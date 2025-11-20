use crate::{
    BaseDataModel, FactorId, Parameter, ParameterKind, Storable, TopicId,
};
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct TopicFactorParameter {
    pub kind: Option<ParameterKind>,
    pub topic_id: Option<TopicId>,
    pub factor_id: Option<FactorId>,
}

impl TopicFactorParameter {
    pub fn init() -> Self {
        TopicFactorParameter::new().kind(ParameterKind::Topic)
    }

    pub fn to_parameter(self) -> Parameter {
        Parameter::Topic(self)
    }
}
