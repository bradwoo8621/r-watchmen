use crate::{ArcHelper, ArcParameterJoint, ArcPipelineStage, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{
    Pipeline, PipelineId, PipelineTriggerType, StdErrorCode, StdR, TenantId, TopicId,
};

#[derive(Debug)]
pub struct ArcPipeline {
    pub pipeline_id: Arc<PipelineId>,
    pub topic_id: Arc<TopicId>,
    pub name: Arc<String>,
    pub r#type: Arc<PipelineTriggerType>,
    pub stages: Arc<Vec<Arc<ArcPipelineStage>>>,
    pub enabled: bool,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
    pub tenant_id: Arc<TenantId>,
    pub version: Option<u32>,
}

impl ArcHelper for ArcPipeline {}

impl ArcPipeline {
    pub fn new(pipeline: Pipeline) -> StdR<Arc<Self>> {
        if pipeline.pipeline_id.is_none() {
            return RuntimeModelKernelErrorCode::PipelineIdMissed.msg("Pipeline must have an id.");
        }

        if pipeline.name.is_none() {
            return RuntimeModelKernelErrorCode::NameMissed.msg("Pipeline must have a name.");
        }
        let name = Arc::new(pipeline.name.unwrap());

        if pipeline.tenant_id.is_none() {
            return RuntimeModelKernelErrorCode::NameMissed
                .msg(format!("Topic[{}] has not tenant.", name));
        }
        let tenant_id = Arc::new(pipeline.tenant_id.unwrap());
        let topic_id = Self::topic_id(pipeline.topic_id, || format!("Pipeline[{}]", name))?;

        if pipeline.r#type.is_none() {
            return RuntimeModelKernelErrorCode::PipelineTypeMissed
                .msg(format!("Pipeline[{}] has no type.", name));
        }

        if pipeline.stages.is_none() {
            return RuntimeModelKernelErrorCode::PipelineStageMissed
                .msg(format!("Pipeline[{}] has no stage.", name));
        }
        let stages = pipeline.stages.unwrap();
        if stages.len() == 0 {
            return RuntimeModelKernelErrorCode::PipelineStageMissed
                .msg(format!("Pipeline[{}] has no stage.", name));
        }
        let mut arc_stages = vec![];
        for stage in stages {
            arc_stages.push(ArcPipelineStage::new(stage)?);
        }
        let arc_stages = Arc::new(arc_stages);

        let on = Self::conditional(pipeline.conditional, pipeline.on, || {
            format!(
                "Pipeline[{}] has no condition when conditional is true.",
                name
            )
        })?;

        Ok(Arc::new(Self {
            pipeline_id: Arc::new(pipeline.pipeline_id.unwrap()),
            topic_id,
            name,
            r#type: Arc::new(pipeline.r#type.unwrap()),
            stages: arc_stages,
            enabled: pipeline.enabled.unwrap_or(true),
            conditional: on.is_some(),
            on,
            tenant_id,
            version: pipeline.version,
        }))
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn is_conditional(&self) -> bool {
        self.conditional
    }
}
