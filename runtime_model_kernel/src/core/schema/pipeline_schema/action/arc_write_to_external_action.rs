use watchmen_model::{WriteToExternalAction, StdR};

#[derive(Debug)]
pub struct ArcWriteToExternalAction {}

impl ArcWriteToExternalAction {
    pub fn new(action: WriteToExternalAction) -> StdR<Self> {}
}
