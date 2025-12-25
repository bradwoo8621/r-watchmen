use crate::{
	Auditable, BaseDataModel, ConnectedSpaceId, LastVisit, SpaceId, Storable, SubjectWithReports,
	TenantId, UserBasedTuple, UserId,
};
use watchmen_base::serde::option_naive_datetime;
use watchmen_model_marco::adapt_model;

/// extend a [subjects] field from [ConnectedSpace]
#[adapt_model(user_based, audit, last_visit)]
pub struct ConnectedSpaceWithSubjects {
    pub connect_id: Option<ConnectedSpaceId>,
    pub space_id: Option<SpaceId>,
    pub name: Option<String>,
    pub is_template: Option<bool>,
    pub subjects: Option<Vec<SubjectWithReports>>,
}
