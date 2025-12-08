use crate::{ArcTopicDataValue, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_model::StdR;

pub trait InMemoryParameter {
    fn value_from(&self, variables: &PipelineExecutionVariables) -> StdR<Arc<ArcTopicDataValue>>;
}
