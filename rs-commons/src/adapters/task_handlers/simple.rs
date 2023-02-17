use deadpool_postgres::Transaction;
use async_trait::async_trait;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::models::worker::task_worker_result::TaskWorkerResult;
use crate::adapters::task_handlers::TaskHandlerTrait;
use crate::db::services::DbServices;

pub struct SimpleHandler {}
impl SimpleHandler {
    pub fn new() -> Self { SimpleHandler{} }
}

#[async_trait]
impl TaskHandlerTrait for SimpleHandler {
    async fn process(&self, _task_worker: TaskWorker, _dbs: &DbServices, args: Option<Vec<TaskVariable>>, _tr: &Transaction<'_>) -> Result<TaskWorkerResult, ErrorDefinition> {
        Ok(TaskWorkerResult::ok_with_args(args.unwrap_or(vec!())))
    }
}