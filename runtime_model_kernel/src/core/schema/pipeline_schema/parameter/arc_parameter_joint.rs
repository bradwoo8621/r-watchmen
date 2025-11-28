use std::sync::Arc;
use watchmen_model::{ParameterJoint, ParameterJointType, StdR};

#[derive(Debug)]
pub struct ArcParameterJoint {
    pub joint_type: Arc<ParameterJointType>,
    pub filters: Arc<Vec<Arc<ArcParameterCondition>>>,
}

impl ArcParameterJoint {
    pub fn new(_joint: ParameterJoint) -> StdR<Arc<ArcParameterJoint>> {
        // TODO
        Ok(Arc::new(Self {}))
    }
}
