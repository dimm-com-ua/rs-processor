use serde_json::Value;
use crate::adapters::models::data_type::DataType;

pub struct TaskVariable {
    pub id: uuid::Uuid,
    pub name: String,
    pub data_type: DataType,
    pub value: Value
}
