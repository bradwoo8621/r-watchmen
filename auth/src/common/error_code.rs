use watchmen_model::StdErrorCode;

pub enum AuthErrorCode {
    AuthenticationFailed,
    Unauthorized,
    Forbidden,
    // user related
    TenantIdMissedInUser,
    UserIdMissedInUser,
    NameMissedInUser,
    RoleMissedInUser,
}

impl StdErrorCode for AuthErrorCode {
    fn code(&self) -> &'static str {
        match self {
            AuthErrorCode::AuthenticationFailed => "AUTH-00001",
            AuthErrorCode::Unauthorized => "AUTH-00002",
            AuthErrorCode::Forbidden => "AUTH-00003",
            AuthErrorCode::TenantIdMissedInUser => "AUTH-00101",
            AuthErrorCode::UserIdMissedInUser => "AUTH-00102",
            AuthErrorCode::NameMissedInUser => "AUTH-00103",
            AuthErrorCode::RoleMissedInUser => "AUTH-00104",
        }
    }
}
