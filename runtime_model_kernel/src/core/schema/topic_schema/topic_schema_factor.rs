use crate::{FakeTopicSchemaFactor, SimpleTopicSchemaFactor, VecOrMapTopicSchemaFactor};
use std::fmt::{Display, Formatter};

pub enum TopicSchemaFactor {
    Simple(SimpleTopicSchemaFactor),
    VecOrMap(VecOrMapTopicSchemaFactor),
    Fake(FakeTopicSchemaFactor),
}

impl Display for TopicSchemaFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Simple(v) => write!(f, "{}", v),
            Self::VecOrMap(v) => write!(f, "{}", v),
            Self::Fake(v) => write!(f, "{}", v),
        }
    }
}

impl TopicSchemaFactor {
    pub fn if_functional(self) -> Option<TopicSchemaFactor> {
        match self {
            Self::Simple(v) => v.if_functional(),
            Self::VecOrMap(v) => v.if_functional(),
            Self::Fake(v) => v.if_functional(),
        }
    }
}
