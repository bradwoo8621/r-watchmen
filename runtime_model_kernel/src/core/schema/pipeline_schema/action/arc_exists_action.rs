use std::sync::Arc;
use watchmen_model::{ExistsAction, ParameterJoint, PipelineActionId, PipelineActionType, StdR, TopicId};
use crate::ArcParameterJoint;

#[derive(Debug)]
pub struct ArcExistsAction {
    pub action_id: Arc<PipelineActionId>,
    pub r#type: Arc<PipelineActionType>,
    /// read from topic
    pub topic_id: Arc<TopicId>,
    /// read criteria
    pub by: Arc<ArcParameterJoint>,
    /// copy to memory variable
    pub variable_name: Arc<String>,
}

impl ArcExistsAction {
    pub fn new(action: ExistsAction) -> StdR<Self> {
        let action_id = action.action_id.unwrap_or("".to_string());


    }
}
