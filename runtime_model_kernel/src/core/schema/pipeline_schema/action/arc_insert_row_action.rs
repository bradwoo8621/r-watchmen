use watchmen_model::{InsertRowAction, StdR};

#[derive(Debug)]
pub struct ArcInsertRowAction {}

impl ArcInsertRowAction {
    pub fn new(action: InsertRowAction) -> StdR<Self> {}
}
