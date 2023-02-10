use crate::services::task_engine_service::TaskEngineService;

pub mod task_engine_service;

#[derive(Clone)]
pub struct EngineServices {
    pub task: TaskEngineService
}

impl EngineServices {
    pub fn new() -> Self {
        EngineServices {
            task: TaskEngineService::new()
        }
    }
}