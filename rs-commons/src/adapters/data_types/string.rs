use serde_json::Value;
use crate::adapters::data_types::DataTypeTrait;

#[derive(Clone)]
pub struct StringDataType {}

impl StringDataType {
    pub fn new() -> Self { StringDataType{} }
}

impl DataTypeTrait for StringDataType {
    fn validate(&self, value: Value) -> Result<(), ()> {
        Ok(())
    }
}
