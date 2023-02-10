use chrono::{DateTime, Utc};
use postgres_types::{ToSql, FromSql};
use serde_json::Value;

#[derive(Debug, ToSql, FromSql, PartialEq, Clone)]
#[postgres(name="pc_process_definition")]
pub struct ProcessDefinitionDb {
    pub id: uuid::Uuid,
    pub name: String,
    pub enabled: bool,
    pub code: String
}

#[derive(Debug, ToSql, FromSql, PartialEq, Clone)]
#[postgres(name="pc_process_definition_flow")]
pub struct ProcessDefinitionFlowDb {
    pub id: uuid::Uuid,
    pub process_id: uuid::Uuid,
    pub version_id: i32,
    pub created_at: Option<DateTime<Utc>>
}

#[derive(Debug, ToSql, FromSql, PartialEq, Clone)]
#[postgres(name="pc_process_flow_element")]
pub struct FlowElementDb {
    pub id: uuid::Uuid,
    pub process_flow: uuid::Uuid,
    pub el_type: String,
    pub handler_type: i32,
    pub handler_value: Value,
    pub description: Option<String>
}