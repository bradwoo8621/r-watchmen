use crate::{Factor, FactorIndexGroup, FactorType, Topic, TopicKind, TopicType};

pub fn ask_pipeline_monitor_error_log_topic() -> Topic {
    let mut factors = Vec::new();
    factors.push(
        Factor::new()
            .factor_id(String::from("rpml-f-1"))
            .name(String::from("traceId"))
            .r#type(FactorType::Text)
            .index_group(FactorIndexGroup::Index1),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("rpml-f-2"))
            .name(String::from("dataId"))
            .r#type(FactorType::Text)
            .index_group(FactorIndexGroup::Index2),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("rpml-f-3"))
            .name(String::from("status"))
            .r#type(FactorType::Text),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("rpml-f-4"))
            .name(String::from("topicId"))
            .r#type(FactorType::Text),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("rpml-f-5"))
            .name(String::from("pipelineId"))
            .r#type(FactorType::Text),
    );
    factors.push(
        Factor::new()
            .factor_id(String::from("rpml-f-6"))
            .name(String::from("uid"))
            .r#type(FactorType::Text)
            .index_group(FactorIndexGroup::Index3),
    );

    Topic::new()
        .name(String::from("pipeline_monitor_error_log"))
        .kind(TopicKind::System)
        .r#type(TopicType::Distinct)
        .factors(factors)
        .description(String::from("Pipeline error monitor"))
        .build()
}
