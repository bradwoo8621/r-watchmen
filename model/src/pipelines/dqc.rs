use crate::{Factor, ParameterKind, Pipeline, Topic, TopicFactorParameter};

fn find_topic<'a>(topics: &'a Vec<Topic>, topic_name: &'static str) -> Option<&'a Topic> {
    topics.into_iter().find(|topic| {
        if let Some(name) = &topic.name {
            name == topic_name
        } else {
            false
        }
    })
}

fn find_factor<'a>(topic: &'a Topic, factor_name: &'static str) -> Option<&'a Factor> {
    if let Some(factors) = &topic.factors {
        factors.into_iter().find(|factor| {
            if let Some(name) = &factor.name {
                name == factor_name
            } else {
                false
            }
        })
    } else {
        None
    }
}

fn find_source_parameter(topic: Topic, factor_name: &'static str) -> Option<TopicFactorParameter> {
    if let Some(factor) = find_factor(&topic, factor_name) {
        Some(TopicFactorParameter {
            kind: Some(ParameterKind::Topic),
            topic_id: topic.topic_id.clone(),
            factor_id: factor.factor_id.clone(),
        })
    } else {
        None
    }
}

pub fn ask_dqc_pipelines(topics: &Vec<Topic>) -> Vec<Pipeline> {
    // define all dqc pipelines
    let topic_raw = find_topic(topics, "dqc_raw_rule_result");
    let topic_daily = find_topic(topics, "dqc_rule_daily");

    // Pipeline::new()
    //     .name(String::from("DQC Pipeline"))
    //     .topic(topic_raw.topicId)
    //     .r#type(PipelineTriggerType::InsertOrMerge)

    Vec::<Pipeline>::new()
}
