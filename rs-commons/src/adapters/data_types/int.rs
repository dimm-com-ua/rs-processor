use crate::adapters::data_types::DataTypeTrait;
use serde_json::Value;

pub struct IntDataType;

impl IntDataType {
    pub fn new() -> Self {
        IntDataType {}
    }
}

impl DataTypeTrait for IntDataType {
    fn validate(&self, value: &Value) -> Result<(), ()> {
        if value.is_i64() {
            Ok(())
        } else {
            Err(())
        }
    }
}
