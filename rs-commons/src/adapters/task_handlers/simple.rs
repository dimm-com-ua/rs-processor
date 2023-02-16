use deadpool_postgres::Transaction;
use async_trait::async_trait;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::task_handlers::TaskHandlerTrait;
use crate::db::services::DbServices;

pub struct SimpleHandler {}
impl SimpleHandler {
    pub fn new() -> Self { SimpleHandler{} }
}

#[async_trait]
impl TaskHandlerTrait for SimpleHandler {
    async fn process(&self, task_worker: TaskWorker, dbs: &DbServices, args: Option<Vec<TaskVariable>>, tr: &Transaction<'_>) -> Result<TaskWorker, ErrorDefinition> {

        Ok(task_worker)
    }
}