use crate::serde::option_naive_datetime;
use crate::{Auditable, BaseDataModel, OptimisticLock, Storable, TenantId, Tuple, UserId};
use watchmen_model_marco::adapt_model;

#[adapt_model(opt_lock, tuple)]
pub struct Tenant {
    pub tenant_id: Option<TenantId>,
    pub name: Option<String>,
    pub enable_a_i: Option<bool>,
}
