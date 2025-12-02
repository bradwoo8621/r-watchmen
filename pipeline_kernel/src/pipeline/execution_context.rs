use crate::{PipelineExecution, PipelineTrigger, TopicTrigger};
use std::sync::Arc;
use watchmen_runtime_model_kernel::PipelineSchema;

pub struct PipelineExecutionContext {
    queue: Vec<PipelineExecution>,
}

impl PipelineExecutionContext {
    pub fn new(
        pipeline_trigger: &PipelineTrigger,
        topic_trigger: Arc<TopicTrigger>,
        pipelines: Vec<Arc<PipelineSchema>>,
    ) -> Self {
        PipelineExecutionContext {
            queue: pipelines
                .into_iter()
                .map(|pipeline_schema| PipelineExecution {
                    topic_schema: pipeline_trigger.topic_schema.clone(),
                    topic_trigger: topic_trigger.clone(),
                    pipeline_schema,
                    // env
                    principal: pipeline_trigger.principal.clone(),
                    trace_id: pipeline_trigger.trace_id.clone(),
                    execution_log_monitor: pipeline_trigger.execution_log_monitor.clone(),
                })
                .collect(),
        }
    }
}
