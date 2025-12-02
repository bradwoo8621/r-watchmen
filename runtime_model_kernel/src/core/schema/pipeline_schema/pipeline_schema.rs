use crate::ArcPipeline;
use std::sync::Arc;
use watchmen_model::{Pipeline, PipelineTriggerType, StdR};

pub struct PipelineSchema {
    inner: Arc<ArcPipeline>,
}

impl PipelineSchema {
    pub fn new(pipeline: Pipeline) -> StdR<Self> {
        Ok(PipelineSchema {
            inner: ArcPipeline::new(pipeline)?,
        })
    }

    pub fn pipeline(&self) -> &Arc<ArcPipeline> {
        &self.inner
    }

    pub fn name(&self) -> &Arc<String> {
        &self.pipeline().name
    }

    pub fn r#type(&self) -> &Arc<PipelineTriggerType> {
        &self.pipeline().r#type
    }
}
