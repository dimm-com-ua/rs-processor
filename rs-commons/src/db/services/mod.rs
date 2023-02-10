use std::sync::Arc;
use crate::adapters::data_types::{DataTypes, DataTypeTrait};
use crate::db::services::flow_db_service::FlowDbService;
use crate::db::services::process_db_service::ProcessDbService;
use crate::db::services::task_db_service::TasksDbService;

pub mod flow_db_service;
pub mod task_db_service;
pub mod process_db_service;

#[derive(Debug)]
pub enum DbServiceError {
    NotFound,
    QueryError(String)
}

#[derive(Clone)]
pub struct DbServices {
    pub tasks: TasksDbService,
    pub process: ProcessDbService,
    pub flow: FlowDbService
}

impl DbServices {
    pub fn new() -> Self {
        DbServices {
            tasks: TasksDbService::new(),
            process: ProcessDbService::new(),
            flow: FlowDbService::new()
        }
    }
}

#[derive(Clone)]
pub struct App {
    data_types: DataTypes
}

pub enum AppError { DataTypeNotFound }

impl App {
    pub fn new() -> Self {
        let mut dt = DataTypes::new();
        dt.init();
        App {
            data_types: dt
        }
    }

    pub fn dt(&self, data_type_name: String) -> Option<&Arc<dyn DataTypeTrait + Sync + Send>> {
        self.data_types.get(data_type_name)
    }
}