use crate::{bdm, ParameterExpression, ParameterJoint};

pub enum ParameterCondition {
    Exp(ParameterExpression),
    Joint(ParameterJoint),
}

bdm!(ParameterCondition);
