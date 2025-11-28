use watchmen_model::{ReadFactorsAction, StdR};

#[derive(Debug)]
pub struct ArcReadFactorsAction {}

impl ArcReadFactorsAction {
    pub fn new(action: ReadFactorsAction) -> StdR<Self> {}
}
