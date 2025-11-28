use watchmen_model::{InsertOrMergeRowAction, StdR};

#[derive(Debug)]
pub struct ArcInsertOrMergeRowAction {}

impl ArcInsertOrMergeRowAction {
    pub fn new(action: InsertOrMergeRowAction) -> StdR<Self> {}
}
