use std::sync::Arc;
use watchmen_model::{
	Parameter, StdR
	,
};

#[derive(Debug)]
pub enum ArcParameter {
    Topic(ArcTopicFactorParameter),
    Constant(ArcConstantParameter),
    Computed(ArcComputedParameter),
}

impl ArcParameter {
    pub fn new(_parameter: Parameter) -> StdR<Arc<ArcParameter>> {
        // TODO
        Ok(Arc::new(Self {}))
    }
}
