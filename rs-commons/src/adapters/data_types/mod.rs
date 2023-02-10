use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use serde_json::Value;
use crate::adapters::data_types::string::StringDataType;

pub trait DataTypeTrait {
    fn validate(&self, value: Value) -> Result<(), ()>;
}

pub mod string;
pub mod number;
pub mod date;
pub mod datetime;
pub mod object;

#[derive(Clone)]
pub struct DataTypes {
    h: HashMap<String, Arc<(dyn DataTypeTrait + Sync + Send)>>
}

impl DataTypes {
    pub fn new() -> Self { DataTypes { h: Default::default() }}
    
    pub fn init(&mut self) {
        self.h = HashMap::new();
        self.h.insert("string".to_string(), Arc::new(StringDataType::new()));
    }

    pub fn get(&self, name: String) -> Option<&Arc<(dyn DataTypeTrait + Sync + Send)>> {
        self.h.get(name.as_str())
    }
}