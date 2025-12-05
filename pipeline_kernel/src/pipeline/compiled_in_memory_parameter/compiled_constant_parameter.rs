use crate::{InMemoryParameter, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_model::{StdR, TopicDataValue};
use watchmen_runtime_model_kernel::ArcConstantParameter;

pub struct CompiledConstantParameter {}

impl CompiledConstantParameter {
    pub fn new(parameter: Arc<ArcConstantParameter>) -> StdR<Self> {
        Ok(CompiledConstantParameter {})
    }
}

impl InMemoryParameter for CompiledConstantParameter {
    fn value_from(&self, _variables: &PipelineExecutionVariables) -> &TopicDataValue {
        todo!("implement value_from for CompiledConstantParameter")
    }
}
