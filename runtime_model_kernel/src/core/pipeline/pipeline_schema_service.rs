use crate::{PipelineMetaService, PipelineSchema, TenantBasedProvider};
use std::sync::Arc;
use watchmen_auth::Principal;
use watchmen_model::{PipelineId, StdR, TenantId, TopicId};

/// TODO pipeline meta service using tenant and it's meta datasource (or the global meta datasource)
///  to find out pipeline meta.
///  the tenant meta datasource is a new feature, which is defined on tenant
pub struct PipelineSchemaService {
    tenant_id: TenantId,
    meta: Arc<PipelineMetaService>,
}

// TODO maybe find from cache
impl PipelineSchemaService {
    pub fn with(tenant_id: &TenantId) -> StdR<Arc<Self>> {
        Ok(Arc::new(Self {
            tenant_id: tenant_id.clone(),
            meta: PipelineMetaService::with(tenant_id)?,
        }))
    }

    pub fn by_pipeline_id(&self, pipeline_id: &PipelineId) -> StdR<Option<Arc<PipelineSchema>>> {
        let pipeline = self.meta.by_pipeline_id(pipeline_id)?;
        if let Some(pipeline) = pipeline {
            let schema = PipelineSchema::new(pipeline)?;
            Ok(Some(Arc::new(schema)))
        } else {
            Ok(None)
        }
    }

    pub fn by_topic_id(&self, topic_id: &TopicId) -> StdR<Option<Vec<Arc<PipelineSchema>>>> {
        let pipelines = self.meta.by_topic_id(topic_id)?;
        match pipelines {
            Some(pipelines) => {
                let mut schemas = vec![];
                for pipeline in pipelines {
                    schemas.push(Arc::new(PipelineSchema::new(pipeline)?));
                }
                Ok(Some(schemas))
            }
            _ => Ok(None),
        }
    }
}

pub trait PipelineSchemaProvider: TenantBasedProvider {
    fn pipeline_schema(&self) -> StdR<Arc<PipelineSchemaService>> {
        PipelineSchemaService::with(self.tenant_id())
    }
}

impl PipelineSchemaProvider for Principal {}
