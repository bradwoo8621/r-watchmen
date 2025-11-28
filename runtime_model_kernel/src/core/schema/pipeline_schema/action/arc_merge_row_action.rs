use watchmen_model::{MergeRowAction, StdR};

#[derive(Debug)]
pub struct ArcMergeRowAction {}

impl ArcMergeRowAction {
    pub fn new(action: MergeRowAction) -> StdR<Self> {}
}
