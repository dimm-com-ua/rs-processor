use crate::adapters::data_types::DataTypeTrait;
use serde_json::Value;

#[derive(Clone)]
pub struct StringDataType {}

impl StringDataType {
    pub fn new() -> Self {
        StringDataType {}
    }
}

impl DataTypeTrait for StringDataType {
    fn validate(&self, value: &Value) -> Result<(), ()> {
        if value.is_string() {
            Ok(())
        } else {
            Err(())
        }
    }
}
