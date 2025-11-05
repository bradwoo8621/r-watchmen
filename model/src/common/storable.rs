use crate::{BaseDataModel, UserId};
use chrono::NaiveDateTime;

pub trait Storable: BaseDataModel {}

/// implements [Storable], also implements [BaseDataModel]
#[macro_export]
macro_rules! storable {
    ($struct_name:ident) => {
        crate::bdm!($struct_name);

        impl crate::Storable for $struct_name {}
    };
}

pub trait Auditable: Storable {
    fn created_at(&self) -> Option<NaiveDateTime>;
    fn created_by(&self) -> Option<UserId>;
    fn last_modified_at(&self) -> Option<NaiveDateTime>;
    fn last_modified_by(&self) -> Option<UserId>;
}

/// implements [Auditable], also implements [Storable]
#[macro_export]
macro_rules! audit {
    ($struct_name:ident) => {
        crate::storable!($struct_name);

        impl crate::Auditable for $struct_name {
            fn created_at(&self) -> Option<chrono::NaiveDateTime> {
                self.created_at
            }

            fn created_by(&self) -> Option<crate::UserId> {
                self.created_by.clone()
            }

            fn last_modified_at(&self) -> Option<chrono::NaiveDateTime> {
                self.last_modified_at
            }

            fn last_modified_by(&self) -> Option<crate::UserId> {
                self.last_modified_by.clone()
            }
        }
    };
}


pub trait OptimisticLock: Storable {
    fn version(&self) -> Option<u32>;
}

/// implements [OptimisticLock]
#[macro_export]
macro_rules! opt_lock {
    ($struct_name:ident) => {
        impl crate::OptimisticLock for $struct_name {
            fn version(&self) -> Option<u32> {
                self.version
            }
        }
    };
}

pub trait LastVisit: Storable {
    fn last_visit_time(&self) -> Option<NaiveDateTime>;
}

/// implements [LastVisit]
#[macro_export]
macro_rules! last_visit {
    ($struct_name:ident) => {
        impl crate::LastVisit for $struct_name {
            fn last_visit_time(&self) -> Option<chrono::NaiveDateTime> {
                self.last_visit_time
            }
        }
    };
}
