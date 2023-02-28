use crate::adapters::data_types::DataTypeTrait;
use serde_json::Value;

pub struct FloatDataType;

impl FloatDataType {
    pub fn new() -> Self {
        FloatDataType {}
    }
}

impl DataTypeTrait for FloatDataType {
    fn validate(&self, value: &Value) -> Result<(), ()> {
        if value.is_f64() {
            Ok(())
        } else {
            Err(())
        }
    }
}
