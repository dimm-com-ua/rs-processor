use crate::adapters::data_types::DataTypeTrait;
use serde_json::Value;

pub struct ObjectDataType;

impl ObjectDataType {
    pub fn new() -> Self {
        ObjectDataType {}
    }
}

impl DataTypeTrait for ObjectDataType {
    fn validate(&self, value: &Value) -> Result<(), ()> {
        if value.is_object() {
            Ok(())
        } else {
            Err(())
        }
    }
}
