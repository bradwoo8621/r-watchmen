use crate::serde::option_naive_datetime;
use crate::{
    Auditable, BaseDataModel, OptimisticLock, Storable, TenantBasedTuple, TenantId, Tuple, UserId,
};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum PluginType {
    Streamlit,
    Jupyter,
}

#[derive(Display, Serde)]
pub enum PluginApplyTo {
    Achievement,
}

pub type PluginId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct Plugin {
    pub plugin_id: Option<PluginId>,
    pub plugin_code: Option<String>,
    pub name: Option<String>,
    pub r#type: Option<PluginType>,
    pub apply_to: Option<PluginApplyTo>,
    /// value is parameter name
    pub params: Option<Vec<String>>,
    /// value is result name
    pub results: Option<Vec<String>>,
}
