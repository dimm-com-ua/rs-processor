use crate::adapters::data_types::float::FloatDataType;
use crate::adapters::data_types::int::IntDataType;
use crate::adapters::data_types::object::ObjectDataType;
use crate::adapters::data_types::string::StringDataType;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;

pub trait DataTypeTrait {
    fn validate(&self, value: &Value) -> Result<(), ()>;
}

pub mod date;
pub mod datetime;
pub mod float;
pub mod int;
pub mod object;
pub mod string;

#[derive(Clone)]
pub struct DataTypes {
    h: HashMap<String, Arc<(dyn DataTypeTrait + Sync + Send)>>,
}

impl DataTypes {
    pub fn new() -> Self {
        DataTypes {
            h: Default::default(),
        }
    }

    pub fn init(&mut self) {
        self.h = HashMap::new();
        self.h
            .insert("string".to_string(), Arc::new(StringDataType::new()));
        self.h
            .insert("int".to_string(), Arc::new(IntDataType::new()));
        self.h
            .insert("float".to_string(), Arc::new(FloatDataType::new()));
        self.h
            .insert("object".to_string(), Arc::new(ObjectDataType::new()));
    }

    pub fn get(&self, name: String) -> Option<&Arc<(dyn DataTypeTrait + Sync + Send)>> {
        self.h.get(name.as_str())
    }
}
