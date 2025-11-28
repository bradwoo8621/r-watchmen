use crate::{ArcHelper, ArcParameterJoint, ArcPipelineAction, RuntimeModelKernelErrorCode};
use std::sync::Arc;
use watchmen_model::{PipelineUnit, PipelineUnitId, StdErrorCode, StdR};

#[derive(Debug)]
pub struct ArcPipelineUnit {
    pub unit_id: Arc<PipelineUnitId>,
    pub name: Arc<String>,
    pub loop_variable_name: Option<Arc<String>>,
    pub r#do: Arc<Vec<Arc<ArcPipelineAction>>>,
    pub conditional: bool,
    pub on: Option<Arc<ArcParameterJoint>>,
}

impl ArcHelper for ArcPipelineUnit {}

impl ArcPipelineUnit {
    pub fn new(unit: PipelineUnit) -> StdR<Arc<Self>> {
        let unit_id = Self::or_empty_str(unit.unit_id);

        // TIP a default name will be generated if there is no name on unit
        let name = Arc::new(unit.name.unwrap_or(String::from("unnamed-unit")));

        if unit.r#do.is_none() {
            return RuntimeModelKernelErrorCode::PipelineActionMissed
                .msg(format!("Pipeline unit[{}] has no action.", unit_id));
        }
        let actions = unit.r#do.unwrap();
        if actions.len() == 0 {
            return RuntimeModelKernelErrorCode::PipelineActionMissed
                .msg(format!("Pipeline unit[{}] has no action.", unit_id));
        }
        let mut arc_actions = vec![];
        for action in actions {
            arc_actions.push(ArcPipelineAction::new(action)?);
        }
        let arc_actions = Arc::new(arc_actions);

        let on = Self::conditional(unit.conditional, unit.on, || {
            format!(
                "Pipeline unit[{}] has no condition when conditional is true.",
                unit_id
            )
        })?;

        Ok(Arc::new(Self {
            unit_id,
            name,
            loop_variable_name: unit.loop_variable_name.map(Arc::new),
            r#do: arc_actions,
            conditional: on.is_some(),
            on,
        }))
    }
}
