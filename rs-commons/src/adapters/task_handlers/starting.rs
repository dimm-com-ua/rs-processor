use deadpool_postgres::Transaction;
use async_trait::async_trait;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::models::worker::task_worker_result::TaskWorkerResult;
use crate::adapters::task_handlers::TaskHandlerTrait;
use crate::db::services::DbServices;

pub struct StartingHandler {}
impl StartingHandler {
    pub fn new() -> Self { StartingHandler{} }
}

#[async_trait]
impl TaskHandlerTrait for StartingHandler {

}