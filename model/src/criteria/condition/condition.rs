use crate::{ParameterExpression, ParameterJoint};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::VariousStructTypes;

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(untagged)]
pub enum ParameterCondition {
    Expression(ParameterExpression),
    Joint(ParameterJoint),
}
