use crate::{Auditable, BaseDataModel, OptimisticLock, Storable, Tuple, UserId};
use watchmen_base::serde::option_naive_datetime;
use watchmen_model_marco::adapt_model;

pub type TenantId = String;

#[adapt_model(opt_lock, tuple)]
pub struct Tenant {
    pub tenant_id: Option<TenantId>,
    pub name: Option<String>,
    pub enable_a_i: Option<bool>,
}
