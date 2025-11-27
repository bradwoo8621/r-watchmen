use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::PipelineTriggerTraceId;

pub struct PipelineExecutionLogMonitor {
    pub principal: Arc<Principal>,
    pub trace_id: Arc<PipelineTriggerTraceId>,
}
