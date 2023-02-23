use crate::adapters::data_types::DataTypeTrait;
use serde_json::Value;

pub struct NumberDataType;

impl NumberDataType {
    pub fn new() -> Self {
        NumberDataType {}
    }
}

impl DataTypeTrait for NumberDataType {
    fn validate(&self, value: &Value) -> Result<(), ()> {
        if value.is_number() {
            Ok(())
        } else {
            Err(())
        }
    }
}
