use watchmen_base::StdR;
use watchmen_model::TopicDataValue;

pub type TriedTopicDataValue = StdR<Option<TopicDataValue>>;

pub trait TriedTDV {
    fn is_untreated(&self) -> bool;

    fn is_treated(&self) -> bool;

    fn is_err(&self) -> bool;

    fn treated_or_default(self, default_value: TopicDataValue) -> TriedTopicDataValue;
}

impl TriedTDV for TriedTopicDataValue {
    fn is_untreated(&self) -> bool {
        self.is_ok() && self.as_ref().unwrap().is_none()
    }

    fn is_treated(&self) -> bool {
        self.is_ok() && !self.as_ref().unwrap().is_none()
    }

    fn is_err(&self) -> bool {
        self.is_err()
    }

    fn treated_or_default(self, default_value: TopicDataValue) -> TriedTopicDataValue {
        let value = self?;
        if let Some(value) = value {
            Ok(Some(value))
        } else {
            Ok(Some(default_value))
        }
    }
}
