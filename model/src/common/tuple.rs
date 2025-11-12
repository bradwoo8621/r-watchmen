use crate::{Auditable, Storable, TenantId, UserId};

pub trait Tuple: Auditable {}

pub trait TenantBasedTuple: Tuple {
    fn tenant_id(&self) -> Option<TenantId>;
}

pub trait UserBasedTuple: Storable {
    fn tenant_id(&self) -> Option<TenantId>;
    fn user_id(&self) -> Option<UserId>;
}
