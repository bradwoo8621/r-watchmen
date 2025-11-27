use crate::serde::option_naive_datetime;
use crate::{
	Auditable, BaseDataModel, ConvergenceId, ObjectiveId, OptimisticLock, SpaceId, Storable,
	TenantBasedTuple, TenantId, Tuple, UserId,
};
use watchmen_model_marco::adapt_model;

pub type UserGroupId = String;

#[adapt_model(opt_lock, tenant_based)]
pub struct UserGroup {
    pub user_group_id: Option<UserGroupId>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub user_ids: Option<Vec<UserId>>,
    pub space_ids: Option<Vec<SpaceId>>,
    pub objective_ids: Option<Vec<ObjectiveId>>,
    pub convergence_ids: Option<Vec<ConvergenceId>>,
}
