use super::parameter::Parameter;
use crate::common::tuple_ids::{FactorId, TopicId};
use crate::criteria::parameter_kind::ParameterKind;

pub trait TopicFactorParameter: Parameter {
    fn kind(&self) -> Option<ParameterKind> {
        Some(ParameterKind::Topic)
    }

    fn topic_id(&self) -> Option<TopicId>;
    fn factor_id(&self) -> Option<FactorId>;
}
