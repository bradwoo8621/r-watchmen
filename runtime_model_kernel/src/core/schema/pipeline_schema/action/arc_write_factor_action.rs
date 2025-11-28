use watchmen_model::{WriteFactorAction, StdR};

#[derive(Debug)]
pub struct ArcWriteFactorAction {}

impl ArcWriteFactorAction {
    pub fn new(action: WriteFactorAction) -> StdR<Self> {}
}
