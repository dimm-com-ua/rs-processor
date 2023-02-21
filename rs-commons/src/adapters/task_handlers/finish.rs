use async_trait::async_trait;
use deadpool_postgres::Transaction;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::models::worker::task_worker_result::TaskWorkerResult;
use crate::adapters::task_handlers::TaskHandlerTrait;
use crate::db::services::{App, DbServices};

pub struct FinishHandler;

impl FinishHandler {
    pub fn new() -> Self { FinishHandler {} }
}

#[async_trait]
impl TaskHandlerTrait for FinishHandler {
    async fn process(&self, task_worker: TaskWorker, dbs: &DbServices, _app: &App, _args: Option<Vec<TaskVariable>>, tr: &Transaction<'_>) -> Result<TaskWorkerResult, ErrorDefinition> {
        match dbs.tasks.clear_task(task_worker.task_id.clone(), tr).await {
            Ok(_) => {
                Ok(TaskWorkerResult::finish())
            }
            Err(err) => {
                Err(err)
            }
        }
    }
}
