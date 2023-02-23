use crate::db::models::task_worker_db::TaskWorkerDb;
use chrono::{DateTime, Utc};
use derive_more::Display;

pub struct TaskWorker {
    pub id: uuid::Uuid,
    pub task_id: uuid::Uuid,
    pub created_at: DateTime<Utc>,
    pub element_id: uuid::Uuid,
    pub what: WorkerWhat,
}

#[derive(Display)]
pub enum WorkerWhat {
    Process,
    RouteAfter,
}

impl TaskWorker {
    pub fn from_db(model: &TaskWorkerDb) -> Self {
        TaskWorker {
            id: model.id,
            task_id: model.task_id,
            created_at: model.created_at,
            element_id: model.element_id,
            what: match model.what.clone().unwrap_or("process".to_string()).as_str() {
                "process" => WorkerWhat::Process,
                "route_after" => WorkerWhat::RouteAfter,
                _ => WorkerWhat::Process,
            },
        }
    }
}
