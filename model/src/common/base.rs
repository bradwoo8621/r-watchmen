use serde_json::Value;
use std::collections::HashMap;

pub type DataModelValue = Value;

pub trait BaseDataModel {
    fn to_map(&self) -> HashMap<&str, DataModelValue>;
}
