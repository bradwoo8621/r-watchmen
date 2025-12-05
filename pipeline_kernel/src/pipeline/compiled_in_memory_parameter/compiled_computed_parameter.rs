use crate::{InMemoryParameter, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_model::TopicDataValue;
use watchmen_runtime_model_kernel::ArcComputedParameter;

pub struct CompiledComputedParameter {}

impl CompiledComputedParameter {
    pub fn new(parameter: Arc<ArcComputedParameter>) -> Self {
        CompiledComputedParameter {}
    }
}

impl InMemoryParameter for CompiledComputedParameter {
    fn value_from(&self, _variables: &PipelineExecutionVariables) -> &TopicDataValue {
        todo!("implement value_from for CompiledComputedParameter")
    }
}
