use watchmen_model::{ReadRowsAction, StdR};

#[derive(Debug)]
pub struct ArcReadRowsAction {}

impl ArcReadRowsAction {
    pub fn new(action: ReadRowsAction) -> StdR<Self> {}
}
