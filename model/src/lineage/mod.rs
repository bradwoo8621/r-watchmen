use crate::{BaseDataModel, Storable, TenantId};
use std::collections::HashMap;
use watchmen_model_marco::adapt_model;

#[adapt_model(storable)]
pub struct LineageGraphs {
    pub directed: Option<bool>,
    pub multigraph: Option<bool>,
    /// TODO don't know the exact type, since in python, it is [Dict]
    pub graph: Option<HashMap<String, String>>,
    pub tenant_id: Option<TenantId>,
}

#[adapt_model(storable)]
pub struct LineageNodes {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[adapt_model(storable)]
pub struct LineageLinks {
    /// TODO put a placeholder to let the [adapt_model] work
    _placeholder: Option<i8>,
}
