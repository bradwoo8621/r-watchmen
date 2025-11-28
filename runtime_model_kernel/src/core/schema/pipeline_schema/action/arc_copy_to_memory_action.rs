use crate::{ArcHelper, ArcParameter, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    CopyToMemoryAction, PipelineActionId, PipelineActionType, StdErrorCode, StdR,
};

#[derive(Debug)]
pub struct ArcCopyToMemoryAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    pub source: Arc<ArcParameter>,
    pub variable_name: Arc<String>,
}

impl ArcHelper for ArcCopyToMemoryAction {}

impl ArcCopyToMemoryAction {
    pub fn new(action: CopyToMemoryAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        if action.source.is_none() {
            return RuntimeModelKernelErrorCode::ActionSourceMissed.msg(format!(
                "Copy to memory action[{}] must have a source.",
                action_id
            ));
        }
        if action.variable_name.is_none() {
            return RuntimeModelKernelErrorCode::ActionVariableNameMissed.msg(format!(
                "Copy-to-memory action[{}] must have a variable name.",
                action_id
            ));
        }

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::CopyToMemory),
            source: ArcParameter::new(action.source.unwrap())?,
            variable_name: Arc::new(action.variable_name.unwrap()),
        })
    }
}
