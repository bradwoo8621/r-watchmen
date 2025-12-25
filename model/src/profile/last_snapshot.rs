use crate::{BaseDataModel, DashboardId, LastVisit, Storable, TenantId, UserBasedTuple, UserId};
use watchmen_base::serde::option_naive_datetime;
use watchmen_model_marco::adapt_model;

#[adapt_model(user_based, last_visit)]
pub struct LastSnapshot {
    pub language: Option<String>,
    pub last_dashboard_id: Option<DashboardId>,
    pub admin_dashboard_id: Option<DashboardId>,
    pub favorite_pin: Option<bool>,
}
