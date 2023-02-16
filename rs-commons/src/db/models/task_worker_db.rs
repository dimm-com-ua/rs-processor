use postgres_types::{ToSql, FromSql};
use chrono::{DateTime, Utc};

#[derive(Clone, Debug, FromSql, ToSql)]
#[postgres(name="pc_task_worker")]
pub struct TaskWorkerDb {
    #[postgres(name="id")]
    pub id: uuid::Uuid,
    #[postgres(name="task_id")]
    pub task_id: uuid::Uuid,
    #[postgres(name="element_id")]
    pub element_id: uuid::Uuid,
    #[postgres(name="created_at")]
    pub created_at: DateTime<Utc>,
    #[postgres(name="run_after")]
    pub run_after: DateTime<Utc>,
    #[postgres(name="runner_key")]
    pub runner_key: Option<uuid::Uuid>,
    #[postgres(name="locked_by")]
    pub locked_by: Option<DateTime<Utc>>
}