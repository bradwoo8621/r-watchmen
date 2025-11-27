use crate::AuthenticationDetails;
use watchmen_model::User;

pub trait AuthenticationProvider {
    fn accept(&self, details: &AuthenticationDetails) -> bool;
    fn authenticate(&self, details: &AuthenticationDetails) -> Option<User>;
}
