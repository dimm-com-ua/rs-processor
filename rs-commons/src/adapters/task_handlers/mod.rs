use std::collections::HashMap;
use std::sync::Arc;
use deadpool_postgres::Transaction;
use async_trait::async_trait;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::task_handlers::simple::SimpleHandler;
use crate::db::services::DbServices;

pub mod simple;
pub mod starting;

#[async_trait]
pub trait TaskHandlerTrait {
    async fn process(&self, task_worker: TaskWorker, dbs: &DbServices, args: Option<Vec<TaskVariable>>, tr: &Transaction<'_>) -> Result<TaskWorker, ErrorDefinition>;
}

#[derive(Clone)]
pub struct TaskHandlers {
    h: HashMap<String, Arc<(dyn TaskHandlerTrait + Sync + Send)>>
}

impl TaskHandlers {
    pub fn new() -> Self { TaskHandlers { h: Default::default() } }

    pub fn init(&mut self) {
        self.h = HashMap::new();
        self.h.insert("starting".to_string(), Arc::new(SimpleHandler::new()));
        self.h.insert("finish".to_string(), Arc::new(SimpleHandler::new()));
    }

    pub fn get(&self, name: String) -> Option<&Arc<(dyn TaskHandlerTrait + Sync + Send)>> {
        self.h.get(name.as_str())
    }
}