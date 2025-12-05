use std::sync::Arc;
use watchmen_model::{StdR, Topic, TopicCode};

/// TODO topic meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out topic meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct TopicMetaService {}

impl TopicMetaService {
    fn new() -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {}))
    }

    pub fn find_by_code(&self, _code: &TopicCode) -> StdR<Topic> {
        todo!("implement find_topic for TopicMetaService")
    }
}

pub trait TopicMetaProvider {
    fn meta() -> StdR<Arc<TopicMetaService>> {
        TopicMetaService::new()
    }
}
