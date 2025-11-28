use crate::{ArcHelper, ArcParameterJoint, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    DeleteRowsAction, PipelineActionId, PipelineActionType, StdErrorCode, StdR, TopicId,
};

#[derive(Debug)]
pub struct ArcDeleteRowsAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    /// delete from topic
    pub topic_id: Arc<TopicId>,
    /// delete criteria
    pub by: Arc<ArcParameterJoint>,
}

impl ArcHelper for ArcDeleteRowsAction {}

impl ArcDeleteRowsAction {
    pub fn new(action: DeleteRowsAction) -> StdR<Self> {
        let action_id = Self::or_empty_str(action.action_id);
        let topic_id = Self::topic_id(action.topic_id, || {
            format!("Delete rows action[{}]", action_id)
        })?;
        if action.by.is_none() {
            return RuntimeModelKernelErrorCode::ConditionMissed
                .msg(format!("Delete rows action[{}] has no by.", action_id));
        }

        Ok(Self {
            action_id,
            r#type: Arc::new(PipelineActionType::DeleteRows),
            topic_id,
            by: ArcParameterJoint::new(action.by.unwrap())?,
        })
    }
}
