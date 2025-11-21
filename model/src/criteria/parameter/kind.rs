use watchmen_model_marco::{Display, Serde, StrEnum};

#[derive(Display, Serde, StrEnum)]
pub enum ParameterKind {
    Topic,
    Constant,
    Computed,
}
