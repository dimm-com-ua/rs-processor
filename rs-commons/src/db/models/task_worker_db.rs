use chrono::{DateTime, Utc};
use postgres_types::{FromSql, ToSql};

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name = "pc_task_worker")]
pub struct TaskWorkerDb {
    pub id: uuid::Uuid,
    pub task_id: uuid::Uuid,
    pub element_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub run_after: DateTime<Utc>,
    pub runner_key: Option<uuid::Uuid>,
    pub locked_by: Option<DateTime<Utc>>,
    pub what: Option<String>,
}
