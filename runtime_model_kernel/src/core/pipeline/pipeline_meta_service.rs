use crate::TenantBasedProvider;
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{Pipeline, PipelineId, StdR, TenantId, TopicId};

/// TODO pipeline meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out pipeline meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct PipelineMetaService {
    tenant_id: TenantId,
}

impl PipelineMetaService {
    pub fn with(tenant_id: &TenantId) -> StdR<Arc<Self>> {
        // TODO maybe find from cache
        Ok(Arc::new(Self {
            tenant_id: tenant_id.clone(),
        }))
    }

    pub fn by_pipeline_id(&self, _pipeline_id: &PipelineId) -> StdR<Option<Pipeline>> {
        todo!("implement find_by_id for PipelineMetaService")
    }

    pub fn by_topic_id(&self, _topic_id: &TopicId) -> StdR<Option<Vec<Pipeline>>> {
        todo!("implement find_pipeline_by_topic for PipelineMetaService")
    }
}

pub trait PipelineMetaProvider: TenantBasedProvider {
    fn pipeline_meta(&self) -> StdR<Arc<PipelineMetaService>> {
        PipelineMetaService::with(self.tenant_id())
    }
}

impl PipelineMetaProvider for Principal {}
