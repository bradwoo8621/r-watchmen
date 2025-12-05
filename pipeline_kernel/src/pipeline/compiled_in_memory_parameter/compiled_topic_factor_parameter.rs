use crate::{InMemoryParameter, PipelineExecutionVariables};
use std::sync::Arc;
use watchmen_model::{StdR, TopicDataValue};
use watchmen_runtime_model_kernel::{ArcTopicFactorParameter, TopicSchemaProvider, TopicService};

pub struct CompiledTopicFactorParameter {}

impl CompiledTopicFactorParameter {
    pub fn new(parameter: Arc<ArcTopicFactorParameter>) -> StdR<Self> {
        let topic_schema = TopicService::schema()?.by_id(parameter.topic_id.as_ref());
        Ok(CompiledTopicFactorParameter {})
    }
}

impl InMemoryParameter for CompiledTopicFactorParameter {
    fn value_from(&self, _variables: &PipelineExecutionVariables) -> &TopicDataValue {
        todo!("implement value_from for CompiledTopicFactorParameter")
    }
}
