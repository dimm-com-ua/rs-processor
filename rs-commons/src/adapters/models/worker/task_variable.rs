use chrono::{DateTime, NaiveDate};
use rhai::Dynamic;
use crate::adapters::models::data_type::DataType;
use serde_json::Value;

pub struct TaskVariable {
    pub id: uuid::Uuid,
    pub name: String,
    pub data_type: DataType,
    pub value: Value,
}

impl TaskVariable {
    pub fn new(name: String, dt: DataType, value: Value) -> Self {
        TaskVariable {
            id: Default::default(),
            name,
            data_type: dt,
            value,
        }
    }

    pub fn to_engine_value(&self) -> Option<Dynamic> {
        match self.data_type.id.as_str() {
            "string" => Some(Dynamic::from(String::from(self.value.clone().as_str().unwrap()))),
            "number" => Some(Dynamic::from(self.value.clone().as_f64().unwrap())),
            "date" => Some(Dynamic::from(
                NaiveDate::parse_from_str(
                    self.value.clone().as_str().unwrap(),
                    "%Y-%m-%d",
                ))),
            "datetime" => Some(Dynamic::from(
                DateTime::parse_from_str(
                    self.value.clone().as_str().unwrap(),
                    "%Y-%m-%d %H:%M:%S %z",
                ))),
            // "object" =>   scope.push(v.name.clone(), Value::from(v.value.clone().as_object().unwrap())),
            "bool" => Some(Dynamic::from( self.value.clone().as_bool().unwrap())),
            _ => None
        }
    }
}
