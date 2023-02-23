use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;
use deadpool_postgres::Transaction;
use tokio::sync::Mutex;

use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::FlowElement;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::models::worker::task_worker_result::TaskWorkerResult;
use crate::adapters::task_handlers::finish::FinishHandler;
use crate::adapters::task_handlers::script_handler::ScriptHandler;
use crate::adapters::task_handlers::simple::SimpleHandler;
use crate::db::services::{App, DbServices};

mod finish;
mod script_handler;
mod simple;
mod starting;

#[async_trait]
pub trait TaskHandlerTrait {
    async fn process(
        &self,
        _task_worker: TaskWorker,
        _flow_element: &FlowElement,
        _dbs: &DbServices,
        _app: &App,
        args: Option<Vec<TaskVariable>>,
        _tr: &Transaction<'_>,
    ) -> Result<TaskWorkerResult, ErrorDefinition> {
        Ok(TaskWorkerResult::ok_with_args(args.unwrap_or(vec![])))
    }
}

#[derive(Clone)]
pub struct TaskHandlers {
    h: HashMap<String, Arc<Mutex<(dyn TaskHandlerTrait + Sync + Send)>>>,
}

impl TaskHandlers {
    pub fn new() -> Self {
        TaskHandlers {
            h: Default::default(),
        }
    }

    pub fn init(&mut self) {
        self.h = HashMap::new();
        self.h.insert(
            "starting".to_string(),
            Arc::new(Mutex::new(SimpleHandler::new())),
        );
        self.h.insert(
            "continue".to_string(),
            Arc::new(Mutex::new(SimpleHandler::new())),
        );
        self.h.insert(
            "match".to_string(),
            Arc::new(Mutex::new(SimpleHandler::new())),
        );
        self.h.insert(
            "finish".to_string(),
            Arc::new(Mutex::new(FinishHandler::new())),
        );
        self.h.insert(
            "on_error".to_string(),
            Arc::new(Mutex::new(SimpleHandler::new())),
        );
        self.h.insert(
            "script".to_string(),
            Arc::new(Mutex::new(ScriptHandler::new())),
        );
    }

    pub fn get(&self, name: String) -> Option<&Arc<Mutex<(dyn TaskHandlerTrait + Sync + Send)>>> {
        self.h.get(name.as_str())
    }
}
