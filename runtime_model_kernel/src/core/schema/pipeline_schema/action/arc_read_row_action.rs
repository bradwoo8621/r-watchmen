use watchmen_model::{ReadRowAction, StdR};

#[derive(Debug)]
pub struct ArcReadRowAction {}

impl ArcReadRowAction {
    pub fn new(action: ReadRowAction) -> StdR<Self> {}
}
