use crate::PipelineExecutionLogMonitor;
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{PipelineTriggerTraceId, PipelineTriggerType, StdR, TopicData, TopicDataId};
use watchmen_runtime_model_kernel::TopicSchema;

pub struct PipelineTrigger {
    pub topic_schema: TopicSchema,
    pub data: TopicData,
    pub r#type: PipelineTriggerType,
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
    pub execution_log_monitor: PipelineExecutionLogMonitor,
}

impl PipelineTrigger {
    pub fn execute(&self) -> StdR<TopicDataId> {
        todo!("implement execute for PipelineTrigger")
    }

    pub async fn execute_async(&self) -> StdR<TopicDataId> {
        todo!("implement execute_async for PipelineTrigger")
    }
}
