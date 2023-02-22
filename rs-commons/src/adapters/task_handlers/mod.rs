use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Transaction;

use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::models::worker::task_worker_result::TaskWorkerResult;
use crate::adapters::task_handlers::finish::FinishHandler;
use crate::adapters::task_handlers::simple::SimpleHandler;
use crate::db::services::{App, DbServices};

#[macro_use]
pub mod macro_mod;

mod simple;
mod starting;
mod finish;
mod test_handler;

#[async_trait]
pub trait TaskHandlerTrait {
    async fn process(&self, _task_worker: TaskWorker, _dbs: &DbServices, _ыapp: &App, args: Option<Vec<TaskVariable>>, _tr: &Transaction<'_>) -> Result<TaskWorkerResult, ErrorDefinition> {
        Ok(TaskWorkerResult::ok_with_args(args.unwrap_or(vec!())))
    }
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
        self.h.insert("continue".to_string(), Arc::new(SimpleHandler::new()));
        self.h.insert("match".to_string(), Arc::new(SimpleHandler::new()));
        self.h.insert("finish".to_string(), Arc::new(FinishHandler::new()));
        self.h.insert("on_error".to_string(), Arc::new(SimpleHandler::new()));
    }

    pub fn get(&self, name: String) -> Option<&Arc<(dyn TaskHandlerTrait + Sync + Send)>> {
        self.h.get(name.as_str())
    }
}
