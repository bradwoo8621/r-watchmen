use super::{parameter_joint::ParameterJoint, parameter_kind::ParameterKind};
use crate::common::base::BaseDataModel;

pub trait Parameter: BaseDataModel {
    fn kind(&self) -> Option<ParameterKind>;
    fn conditional(&self) -> Option<bool>;
    fn on(&self) -> Option<Box<dyn ParameterJoint>>;
}
