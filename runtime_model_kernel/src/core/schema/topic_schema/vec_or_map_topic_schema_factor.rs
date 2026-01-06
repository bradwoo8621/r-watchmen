use crate::{ArcFactor, TopicSchemaFactor, TopicSchemaFactorUtils};
use std::fmt::{Display, Formatter};
use std::sync::Arc;

pub struct VecOrMapTopicSchemaFactor {
    pub factor: Arc<ArcFactor>,
    pub name: String,
    pub children: Vec<TopicSchemaFactor>,
}

impl Display for VecOrMapTopicSchemaFactor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.children.is_empty() {
            write!(
                f,
                "VecOrMapTopicSchemaFactor[name={}, factor_name={}, factor_id={}, factors=[]]",
                self.name, self.factor.name, self.factor.factor_id,
            )
        } else {
            write!(
                f,
                "VecOrMapTopicSchemaFactor[name={}, factor_name={}, factor_id={}, factors=[\n{}\n]]",
                self.name,
                self.factor.name,
                self.factor.factor_id,
                TopicSchemaFactorUtils::factors_to_str(&self.children)
            )
        }
    }
}

impl VecOrMapTopicSchemaFactor {
    pub fn new(factor: Arc<ArcFactor>, name: String) -> Self {
        Self {
            factor,
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
            Some(TopicSchemaFactor::VecOrMap(Self {
                factor: self.factor,
                name: self.name,
                children: functional_children,
            }))
        }
    }
}
