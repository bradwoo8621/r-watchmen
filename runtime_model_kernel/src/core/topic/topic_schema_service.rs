use crate::{TopicMetaProvider, TopicSchema};
use std::sync::Arc;
use watchmen_model::{StdR, TopicCode};

/// TODO topic meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out topic meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct TopicSchemaService {}

impl TopicMetaProvider for TopicSchemaService {}

impl TopicSchemaService {
    fn new() -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {}))
    }

    pub fn by_code(&self, code: &TopicCode) -> StdR<Arc<TopicSchema>> {
        let topic = Self::meta()?.find_by_code(code)?;
        let schema = TopicSchema::new(topic)?;
        Ok(Arc::new(schema))
    }
}

pub trait TopicSchemaProvider {
    fn schema() -> StdR<Arc<TopicSchemaService>> {
        TopicSchemaService::new()
    }
}
