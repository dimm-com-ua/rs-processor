use postgres_types::{ToSql, FromSql};
use chrono::{DateTime, Utc};
use serde_json::Value;

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name="pc_task")]
pub struct TaskDb {
    pub id: uuid::Uuid,
    pub process_flow: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub current_flow_item: Option<uuid::Uuid>
}

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name="pc_task_variable")]
pub struct TaskVariableDb {
    pub id: uuid::Uuid,
    pub task_id: uuid::Uuid,
    pub name: String,
    pub data_type: String,
    pub value: Value,
    pub flow_element_id: Option<uuid::Uuid>
}