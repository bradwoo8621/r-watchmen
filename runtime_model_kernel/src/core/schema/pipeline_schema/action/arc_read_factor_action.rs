use watchmen_model::{ReadFactorAction, StdR};

#[derive(Debug)]
pub struct ArcReadFactorAction {}

impl ArcReadFactorAction {
    pub fn new(action: ReadFactorAction) -> StdR<Self> {}
}
