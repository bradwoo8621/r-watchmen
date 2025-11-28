use crate::{PipelineExecutionLogMonitor, PipelineKernelErrorCode, TopicTrigger};
use std::ops::Deref;
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{
    PipelineTriggerTraceId, PipelineTriggerType, StdErrorCode, StdR, TopicData, TopicDataId,
    TopicKind, VoidR,
};
use watchmen_runtime_model_kernel::{TopicDataService, TopicSchema};

pub struct PipelineTrigger {
    pub topic_schema: Arc<TopicSchema>,
    pub r#type: PipelineTriggerType,
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
    pub execution_log_monitor: PipelineExecutionLogMonitor,
}

impl PipelineTrigger {
    fn prepare_trigger_data(&self, data: &mut TopicData) -> VoidR {
        self.topic_schema.prepare_data(data)
    }

    fn find_topic_data_service(&self) -> StdR<Arc<TopicDataService>> {
        TopicDataService::with(&self.principal.tenant_id)
    }

    fn save_trigger_data(&mut self, mut data: TopicData) -> StdR<Arc<TopicTrigger>> {
        let topic_kind = &self.topic_schema.topic().kind;
        match topic_kind {
            Some(kind) => match kind.deref() {
                TopicKind::Synonym => {
                    return match self.r#type {
                        PipelineTriggerType::Insert => {
                            // only insertion is supported on synonym
                            // will do nothing on synonym topic itself, simply trigger the insert pipeline
                            // typically, there should a historical topic to handle data from synonym topic
                            // and process data based on historical topic insertion
                            self.prepare_trigger_data(&mut data)?;
                            TopicTrigger::insert_to_synonym(data)
                        }
                        _ => {
                            PipelineKernelErrorCode::TriggerTypeNotSupportedOnSynonym.msg(format!(
                                "Trigger type[{}] is not supported on synonym[{}].",
                                self.r#type,
                                self.topic_schema.topic_name()
                            ))
                        }
                    };
                }
                _ => {}
            },
            _ => {}
        };

        self.prepare_trigger_data(&mut data)?;

        let topic_data_service = self.find_topic_data_service()?;

        match self.r#type {
            PipelineTriggerType::Insert => {
                let current_data = topic_data_service.insert(data)?;
                TopicTrigger::insert(current_data)
            }
            PipelineTriggerType::InsertOrMerge => {
                let (previous_data, current_data) = topic_data_service.insert_or_merge(data)?;
                match previous_data {
                    Some(previous_data) => TopicTrigger::merge(previous_data, current_data),
                    _ => TopicTrigger::insert(current_data),
                }
            }
            PipelineTriggerType::Merge => {
                let (previous_data, current_data) = topic_data_service.merge(data)?;
                TopicTrigger::merge(previous_data, current_data)
            }
            PipelineTriggerType::Delete => {
                let previous_data = topic_data_service.delete(data)?;
                TopicTrigger::delete(previous_data)
            }
        }
    }

    pub fn execute(&mut self, data: TopicData) -> StdR<TopicDataId> {
        let topic_trigger = self.save_trigger_data(data)?;
        Ok(topic_trigger.data_id())
    }

    pub async fn execute_async(&mut self, data: TopicData) -> StdR<TopicDataId> {
        let topic_trigger = self.save_trigger_data(data)?;
        Ok(topic_trigger.data_id())
    }
}
