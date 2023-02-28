use std::f64;
use crate::adapters::models::data_type::DataType;
use chrono::{DateTime, NaiveDate};
use rhai::Dynamic;
use serde_json::Value;

#[derive(Debug)]
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

    pub fn to_engine_value(&self) -> Dynamic {
        match self.data_type.id.as_str() {
            "string" => Dynamic::from(String::from(self.value.clone().as_str().unwrap())),
            "int" => Dynamic::from(self.value.clone().as_i64().unwrap_or(0)),
            "float" => Dynamic::from(self.value.clone().as_f64().unwrap_or(f64::default().try_into().unwrap())),
            "date" => Dynamic::from(NaiveDate::parse_from_str(
                self.value.clone().as_str().unwrap(),
                "%Y-%m-%d",
            )),
            "datetime" => Dynamic::from(DateTime::parse_from_str(
                self.value.clone().as_str().unwrap(),
                "%Y-%m-%d %H:%M:%S %z",
            )),
            // "object" =>   scope.push(v.name.clone(), Value::from(v.value.clone().as_object().unwrap())),
            "bool" => Dynamic::from(self.value.clone().as_bool().unwrap()),
            _ => Dynamic::from(None::<String>),
        }
    }

    pub fn from_dynamic(name: String, value: Dynamic) -> Self {
        let dt_id = if value.is_string() {
            if DateTime::parse_from_str(value.to_string().as_str(), "%Y-%m-%d").is_ok() {
                "date"
            } else if DateTime::parse_from_str(value.to_string().as_str(), "%Y-%m-%d %H:%M:%S %z")
                .is_ok()
            {
                "datetime"
            } else {
                "string"
            }
        } else if value.is_float() {
            "float"
        } else if value.is_int() {
            "int"
        } else if value.is_timestamp() {
            "datetime"
        } else if value.is_bool() {
            "bool"
        } else {
            "string"
        };
        let json_val: Value = serde_json::to_value(&value.to_string()).unwrap();
        TaskVariable {
            id: Default::default(),
            name,
            data_type: DataType {
                id: dt_id.to_string(),
                name: dt_id.to_string(),
                handler: "".to_string(),
            },
            value: json_val,
        }
    }
}
