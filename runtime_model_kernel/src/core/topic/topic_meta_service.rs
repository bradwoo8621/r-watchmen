use crate::TopicSchema;
use std::sync::Arc;
use watchmen_model::{StdR, TenantId, Topic};

/// TODO topic meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out topic meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct TopicMetaService {
    tenant_id: TenantId,
}

impl TopicMetaService {
    pub fn with(tenant_id: &TenantId) -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(TopicMetaService {
            tenant_id: tenant_id.clone(),
        }))
    }

    pub fn find_topic(&self) -> StdR<Topic> {
        todo!("implement find_topic for TopicMetaService")
    }

    pub fn find_topic_schema(&self) -> StdR<Arc<TopicSchema>> {
        Ok(Arc::new(TopicSchema::new(self.find_topic()?)))
    }
}
