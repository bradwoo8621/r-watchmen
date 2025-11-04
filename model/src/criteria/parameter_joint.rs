use super::parameter_condition::ParameterCondition;
use crate::serde_for_enum;
use std::fmt;

pub enum ParameterJointType {
    And,
    Or,
}

impl fmt::Display for ParameterJointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParameterJointType::And => write!(f, "and"),
            ParameterJointType::Or => write!(f, "or"),
        }
    }
}

serde_for_enum! {
    ParameterJointType {
        And => "and",
        Or => "or",
    }
}

pub trait ParameterJoint: ParameterCondition {
    fn joint_type(&self) -> Option<ParameterJointType>;
    fn filters(&self) -> Option<Vec<Box<dyn ParameterCondition>>>;
}
