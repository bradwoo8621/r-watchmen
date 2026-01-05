// use crate::{ArcFactor, ArcTopic, TopicSchemaFactorGroup};
// use std::fmt::{Display, Formatter};
// use std::sync::Arc;
//
// #[derive(Debug)]
// pub struct TopicSchemaFlattenFactor {
//     pub factor: Arc<ArcFactor>,
//     pub factor_name: Arc<String>,
//     pub names: Arc<Vec<String>>,
// }
//
// impl Display for TopicSchemaFlattenFactor {
//     fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "TopicSchemaFlattenFactor[factor_id={}, factor_name={}, names={}]",
//             self.factor.factor_id,
//             self.factor_name,
//             self.names.join(", ")
//         )
//     }
// }
//
// impl TopicSchemaFlattenFactor {
//     pub fn of_factor(factor: &Arc<ArcFactor>) -> Option<Arc<Self>> {
//         if !factor.flatten {
//             return None;
//         }
//     }
//
//     pub fn of_topic(topic: &Arc<ArcTopic>) -> Option<Arc<Vec<Arc<Self>>>> {
//         let mut vec = vec![];
//         for factor in topic.factors.iter() {
//             if let Some(factor) = Self::of_factor(factor) {
//                 vec.push(factor);
//             }
//         }
//         if vec.is_empty() {
//             None
//         } else {
//             Some(Arc::new(vec))
//         }
//     }
// }
//
// #[derive(Debug)]
// pub struct TopicSchemaFlattenFactors {
//
// }
