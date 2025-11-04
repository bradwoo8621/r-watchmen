use super::base::{BaseDataModel, DataModelValue};
use serde_json::Value;
use std::collections::HashMap;

pub trait GraphicPosition: BaseDataModel {
    fn x(&self) -> Option<f32>;
    fn y(&self) -> Option<f32>;
}

pub trait GraphicSize: BaseDataModel {
    fn width(&self) -> Option<f32>;
    fn height(&self) -> Option<f32>;
}

pub struct GraphicRect {
    pub x: Option<f32>,
    pub y: Option<f32>,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl BaseDataModel for GraphicRect {
    fn to_map(&self) -> HashMap<&str, DataModelValue> {
        let mut map = HashMap::new();
        if let Some(x) = self.x {
            map.insert("x", Value::from(x));
        }
        if let Some(y) = self.y {
            map.insert("y", Value::from(y));
        }
        if let Some(width) = self.width {
            map.insert("width", Value::from(width));
        }
        if let Some(height) = self.height {
            map.insert("height", Value::from(height));
        }
        map
    }
}

impl GraphicPosition for GraphicRect {
    fn x(&self) -> Option<f32> {
        self.x
    }

    fn y(&self) -> Option<f32> {
        self.y
    }
}

impl GraphicSize for GraphicRect {
    fn width(&self) -> Option<f32> {
        self.width
    }

    fn height(&self) -> Option<f32> {
        self.height
    }
}
