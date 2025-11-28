use crate::{ArcHelper, ArcParameterJoint, ArcPipelineUnit, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{PipelineStage, PipelineStageId, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcPipelineStage {
    pub stage_id: Arc<PipelineStageId>,
    pub name: Arc<String>,
    pub units: Arc<Vec<Arc<ArcPipelineUnit>>>,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
}

impl ArcHelper for ArcPipelineStage {}

impl ArcPipelineStage {
    pub fn new(stage: PipelineStage) -> StdR<Arc<Self>> {
        let stage_id = Self::or_empty_str(stage.stage_id);
        // TIP a default name will be generated if there is no name on stage
        let name = Arc::new(stage.name.unwrap_or(String::from("unnamed-stage")));

        if stage.units.is_none() {
            return RuntimeModelKernelErrorCode::PipelineUnitMissed
                .msg(format!("Pipeline stage[{}] has no stage.", stage_id));
        }
        let units = stage.units.unwrap();
        if units.len() == 0 {
            return RuntimeModelKernelErrorCode::PipelineUnitMissed
                .msg(format!("Pipeline stage[{}] has no stage.", stage_id));
        }
        let mut arc_units = vec![];
        for unit in units {
            arc_units.push(ArcPipelineUnit::new(unit)?);
        }
        let arc_units = Arc::new(arc_units);

        let on = Self::conditional(stage.conditional, stage.on, || {
            format!(
                "Pipeline stage[{}] has no condition when conditional is true.",
                stage_id
            )
        })?;

        Ok(Arc::new(Self {
            stage_id,
            name,
            units: arc_units,
            conditional: on.is_some(),
            on,
        }))
    }
}
