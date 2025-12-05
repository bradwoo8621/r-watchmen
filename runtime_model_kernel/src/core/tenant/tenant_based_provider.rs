use watchmen_auth::Principal;
use watchmen_model::TenantId;

pub trait TenantBasedProvider {
    /// get tenant
    fn tenant_id(&self) -> &TenantId;
}

impl TenantBasedProvider for Principal {
    fn tenant_id(&self) -> &TenantId {
        &self.tenant_id
    }
}
