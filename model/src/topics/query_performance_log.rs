use crate::{
    FactorBuilder, FactorIndexGroup, FactorType, Topic, TopicBuilder, TopicKind, TopicType,
};

pub struct Pipeline {}

pub fn create_qpt_topic() -> Topic {
    let mut factors = Vec::new();
    factors.push(
        FactorBuilder::new()
            .factor_id(String::from("rmpl-f-1"))
            .name(String::from("uid"))
            .r#type(FactorType::Text)
            .build(),
    );
    factors.push(
        FactorBuilder::new()
            .factor_id(String::from("rmpl-f-2"))
            .name(String::from("topic_dimensions"))
            .r#type(FactorType::Text)
            .flatten(true)
            .index_group(FactorIndexGroup::Index1)
            .precision(String::from("200"))
            .build(),
    );
    factors.push(
        FactorBuilder::new()
            .factor_id(String::from("rmpl-f-3"))
            .name(String::from("column_dimensions"))
            .r#type(FactorType::Text)
            .flatten(true)
            .index_group(FactorIndexGroup::Index2)
            .precision(String::from("200"))
            .build(),
    );
    factors.push(
        FactorBuilder::new()
            .factor_id(String::from("rmpl-f-4"))
            .name(String::from("execution_time"))
            .r#type(FactorType::Number)
            .precision(String::from("50"))
            .build(),
    );
    factors.push(
        FactorBuilder::new()
            .factor_id(String::from("rmpl-f-5"))
            .name(String::from("data_volume"))
            .r#type(FactorType::Number)
            .precision(String::from("50"))
            .build(),
    );
    factors.push(
        FactorBuilder::new()
            .factor_id(String::from("rmpl-f-6"))
            .name(String::from("join_dimensions"))
            .r#type(FactorType::Text)
            .flatten(true)
            .precision(String::from("200"))
            .build(),
    );
    factors.push(
        FactorBuilder::new()
            .factor_id(String::from("rmpl-f-7"))
            .name(String::from("where_dimensions"))
            .r#type(FactorType::Text)
            .flatten(true)
            .precision(String::from("200"))
            .build(),
    );
    factors.push(
        FactorBuilder::new()
            .factor_id(String::from("rmpl-f-8"))
            .name(String::from("group_by_dimensions"))
            .r#type(FactorType::Text)
            .flatten(true)
            .precision(String::from("200"))
            .build(),
    );

    TopicBuilder::new()
        .name(String::from("query_performance_log"))
        .kind(TopicKind::System)
        .r#type(TopicType::Raw)
        .factors(factors)
        .description(String::from("query performance	log	raw	topic"))
        .build()
}

pub fn ask_query_performance_topics() -> Vec<Topic> {
    let mut topics = Vec::new();
    topics.push(create_qpt_topic());
    topics
}

pub fn ask_query_performance_pipelines(_topics: Vec<Topic>) -> Vec<Pipeline> {
    // TODO define all pipeline monitor pipelines
    Vec::new()
}
