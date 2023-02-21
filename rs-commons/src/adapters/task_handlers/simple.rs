use deadpool_postgres::Transaction;
use async_trait::async_trait;
use serde_json::json;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::data_type::DataType;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::models::worker::task_worker_result::TaskWorkerResult;
use crate::adapters::task_handlers::TaskHandlerTrait;
use crate::db::services::{App, DbServices};

pub struct SimpleHandler {}
impl SimpleHandler {
    pub fn new() -> Self { SimpleHandler{} }
}

#[async_trait]
impl TaskHandlerTrait for SimpleHandler {
    async fn process(&self, task_worker: TaskWorker, dbs: &DbServices, _app: &App, args: Option<Vec<TaskVariable>>, tr: &Transaction<'_>) -> Result<TaskWorkerResult, ErrorDefinition> {
        let mut args = args.unwrap_or(vec![]);
        args.push(TaskVariable::new(
            "text".to_string(),
            DataType::from_db(&dbs.core.get_data_type("string".to_string(), tr).await.unwrap()),
            json!("This is text")
        ));
        args.push(TaskVariable::new(
            "text_1".to_string(),
            DataType::from_db(&dbs.core.get_data_type("string".to_string(), tr).await.unwrap()),
            json!(format!("Task id is: {}", task_worker.task_id.clone()))
        ));
        Ok(TaskWorkerResult::ok_with_args(args))
    }
}