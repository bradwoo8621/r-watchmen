use crate::serde::{option_naive_date, option_naive_datetime};
use crate::{BaseDataModel, MonitorRuleStatisticalInterval, Storable, TenantId, TopicId, UserId};
use chrono::{NaiveDate, NaiveDateTime};
use watchmen_model_marco::{adapt_model, Display, Serde};

#[derive(Display, Serde)]
pub enum MonitorJobLockStatus {
    Ready,
    Fail,
    Success,
}

pub type MonitorJobLockId = String;

#[adapt_model(storable)]
pub struct MonitorJobLock {
    pub lock_id: Option<MonitorJobLockId>,
    pub tenant_id: Option<TenantId>,
    pub topic_id: Option<TopicId>,
    pub frequency: Option<MonitorRuleStatisticalInterval>,
    #[serde(with = "option_naive_date")]
    pub process_date: Option<NaiveDate>,
    pub status: Option<MonitorJobLockStatus>,
    pub user_id: Option<UserId>,
    #[serde(with = "option_naive_datetime")]
    pub created_at: Option<NaiveDateTime>,
}
