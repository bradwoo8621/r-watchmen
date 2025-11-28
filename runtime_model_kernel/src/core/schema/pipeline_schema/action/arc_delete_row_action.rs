use crate::{ArcHelper, ArcParameterJoint, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    DeleteRowAction, PipelineActionId, PipelineActionType, StdErrorCode, StdR, TopicId,
};

#[derive(Debug)]
pub struct ArcDeleteRowAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    /// delete from topic
    pub topic_id: Arc<TopicId>,
    /// delete criteria
    pub by: Arc<ArcParameterJoint>,
}

impl ArcHelper for ArcDeleteRowAction {}

impl ArcDeleteRowAction {
    pub fn new(action: DeleteRowAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Delete row action[{}]", action_id)
        })?;
        if action.by.is_none() {
            return RuntimeModelKernelErrorCode::ConditionMissed
                .msg(format!("Delete row action[{}] has no by.", action_id));
        }

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::DeleteRow),
            topic_id,
            by: ArcParameterJoint::new(action.by.unwrap())?,
        })
    }
}
