use crate::serde::option_naive_datetime;
use crate::{opt_lock, serde_for_enum, tenant_base, TenantId, UserGroupId, UserId};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::fmt;

pub enum UserRole {
    Console,
    Admin,
    SuperAdmin,
}

impl fmt::Display for UserRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserRole::Console => write!(f, "console"),
            UserRole::Admin => write!(f, "admin"),
            UserRole::SuperAdmin => write!(f, "superadmin"),
        }
    }
}

serde_for_enum! {
    UserRole {
        Console => "console",
        Admin => "admin",
        SuperAdmin => "superadmin"
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct User {
    pub user_id: Option<UserId>,
    pub name: Option<String>,
    pub nick_name: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub is_active: Option<bool>,
    pub group_ids: Option<Vec<UserGroupId>>,
    pub role: Option<UserRole>,
    pub version: Option<u32>,
    pub tenant_id: Option<TenantId>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "option_naive_datetime"
    )]
    pub created_at: Option<NaiveDateTime>,
    pub created_by: Option<UserId>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "option_naive_datetime"
    )]
    pub last_modified_at: Option<NaiveDateTime>,
    pub last_modified_by: Option<UserId>,
}

tenant_base!(User);
opt_lock!(User);
