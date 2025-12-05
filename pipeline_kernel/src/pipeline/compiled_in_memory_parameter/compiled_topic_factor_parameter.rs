use crate::{InMemoryParameter, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_model::TopicDataValue;
use watchmen_runtime_model_kernel::ArcTopicFactorParameter;

pub struct CompiledTopicFactorParameter {}

impl CompiledTopicFactorParameter {
	pub fn new(parameter: Arc<ArcTopicFactorParameter>) -> Self {
		CompiledTopicFactorParameter {}
	}
}

impl InMemoryParameter for CompiledTopicFactorParameter {
	fn value_from(&self, _variables: &PipelineExecutionVariables) -> &TopicDataValue {
		todo!("implement value_from for CompiledTopicFactorParameter")
	}
}
