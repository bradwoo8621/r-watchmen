use crate::{Auditable, TenantId, UserId};

pub trait Tuple: Auditable {}

/// implements [Tuple], also implements [Auditable]
#[macro_export]
macro_rules! tuple {
    ($struct_name:ident) => {
        crate::audit!($struct_name);

        impl crate::Tuple for $struct_name {}
    };
}

pub trait TenantBasedTuple: Tuple {
    fn tenant_id(&self) -> Option<TenantId>;
}

/// implements [TenantBasedTuple], also implements [Tuple]
#[macro_export]
macro_rules! tenant_base {
    ($struct_name:ident) => {
        crate::tuple!($struct_name);

        impl crate::TenantBasedTuple for $struct_name {
            fn tenant_id(&self) -> Option<crate::TenantId> {
                self.tenant_id.clone()
            }
        }
    };
}

pub trait UserBasedTuple: TenantBasedTuple {
    fn user_id(&self) -> Option<UserId>;
}

/// implements [UserBasedTuple], also implements [TenantBasedTuple]
#[macro_export]
macro_rules! user_base {
    ($struct_name:ident) => {
        crate::tenant_base!($struct_name);

        impl crate::UserBasedTuple for $struct_name {
            fn user_id(&self) -> Option<crate::UserId> {
                self.user_id.clone()
            }
        }
    };
}
