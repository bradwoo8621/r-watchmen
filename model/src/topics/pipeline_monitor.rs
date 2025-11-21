use crate::topics::{ask_pipeline_monitor_error_log_topic, ask_raw_pipeline_monitor_topic};
use crate::Topic;

pub fn ask_pipeline_monitor_topics() -> Vec<Topic> {
    let mut topics = Vec::new();
    topics.push(ask_raw_pipeline_monitor_topic());
    topics.push(ask_pipeline_monitor_error_log_topic());
    topics
}
