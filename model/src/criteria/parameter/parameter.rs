use crate::{ComputedParameter, ConstantParameter, TopicFactorParameter};
use serde::{Deserialize, Serialize};
use watchmen_model_marco::VariousStructTypes;

#[derive(Serialize, Deserialize, VariousStructTypes)]
#[serde(tag = "kind")]
pub enum Parameter {
    #[serde(rename = "topic")]
    Topic(TopicFactorParameter),
    #[serde(rename = "constant")]
    Constant(ConstantParameter),
    #[serde(rename = "computed")]
    Computed(ComputedParameter),
}
