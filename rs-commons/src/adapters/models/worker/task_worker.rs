use chrono::{DateTime, Utc};
use crate::db::models::task_worker_db::TaskWorkerDb;

pub struct TaskWorker {
    pub id: uuid::Uuid,
    pub task_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub element_id: uuid::Uuid
}

impl TaskWorker {
    pub fn from_db(model: TaskWorkerDb) -> Self {
        TaskWorker {
            id: model.id,
            task_id: model.task_id,
            created_at: model.created_at,
            element_id: model.element_id,
        }
    }
}