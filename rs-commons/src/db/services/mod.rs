use crate::adapters::data_types::{DataTypeTrait, DataTypes};
use crate::adapters::js_code::JsCodeService;
use crate::adapters::task_handlers::{TaskHandlerTrait, TaskHandlers};
use crate::db::services::core_db_service::CoreDbService;
use crate::db::services::flow_db_service::FlowDbService;
use crate::db::services::process_db_service::ProcessDbService;
use crate::db::services::task_db_service::TasksDbService;
use crate::db::services::worker_db_service::WorkerDbService;
use std::sync::Arc;
use tokio::sync::Mutex;

pub mod core_db_service;
pub mod flow_db_service;
pub mod process_db_service;
pub mod task_db_service;
pub mod worker_db_service;

#[derive(Debug)]
pub enum DbServiceError {
    NotFound,
    QueryError(String),
}

#[derive(Clone)]
pub struct DbServices {
    pub core: CoreDbService,
    pub tasks: TasksDbService,
    pub process: ProcessDbService,
    pub flow: FlowDbService,
    pub worker: WorkerDbService,
}

impl DbServices {
    pub fn new() -> Self {
        DbServices {
            core: CoreDbService::new(),
            tasks: TasksDbService::new(),
            process: ProcessDbService::new(),
            flow: FlowDbService::new(),
            worker: WorkerDbService::new(),
        }
    }
}

#[derive(Clone)]
pub struct App {
    data_types: DataTypes,
    handlers: TaskHandlers,
    js_code: JsCodeService,
}

pub enum AppError {
    DataTypeNotFound,
}

impl App {
    pub fn new() -> Self {
        let mut dt = DataTypes::new();
        dt.init();
        let mut handlers = TaskHandlers::new();
        handlers.init();
        App {
            data_types: dt,
            handlers,
            js_code: JsCodeService::new(),
        }
    }

    pub fn dt(&self, data_type_name: String) -> Option<&Arc<dyn DataTypeTrait + Sync + Send>> {
        self.data_types.get(data_type_name)
    }

    pub fn handler(
        &self,
        handler_name: String,
    ) -> Option<&Arc<Mutex<dyn TaskHandlerTrait + Sync + Send>>> {
        self.handlers.get(handler_name)
    }

    pub fn js_code(&self) -> Arc<JsCodeService> {
        Arc::new(self.js_code.clone())
    }
}
