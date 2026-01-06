use crate::{TopicSchemaFactor, TopicSchemaFactorUtils};
use std::fmt::{Display, Formatter};

pub struct FakeTopicSchemaFactor {
    pub full_name: String,
    pub name: String,
    pub children: Vec<TopicSchemaFactor>,
}

impl Display for FakeTopicSchemaFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.children.is_empty() {
            write!(f, "FakeTopicSchemaFactor[name={}, factors=[]]", self.name,)
        } else {
            write!(
                f,
                "FakeTopicSchemaFactor[name={}, factors=[\n{}\n]]",
                self.name,
                TopicSchemaFactorUtils::factors_to_str(&self.children)
            )
        }
    }
}

impl FakeTopicSchemaFactor {
    pub fn new(full_name: String, name: String) -> Self {
        Self {
            full_name,
            name,
            children: vec![],
        }
    }

    pub fn if_functional(self) -> Option<TopicSchemaFactor> {
        let functional_children: Vec<TopicSchemaFactor> =
            TopicSchemaFactorUtils::filter_functional_factors(self.children);
        if functional_children.is_empty() {
            None
        } else {
            Some(TopicSchemaFactor::Fake(Self {
                full_name: self.full_name,
                name: self.name,
                children: functional_children,
            }))
        }
    }
}
