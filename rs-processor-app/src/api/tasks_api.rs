use crate::api::models::errors::ProcessorError;
use crate::api::tasks_api::modules::CreateTaskRequest;
use crate::AppService;
use actix_web::{post, web, HttpResponse};
use rs_commons::adapters::models::task::CreateTask;
use std::sync::Arc;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/")).service(create_task);
}

pub mod modules {
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::collections::HashMap;

    #[derive(Deserialize, Serialize)]
    pub struct CreateTaskRequest {
        pub flow: String,
        pub arguments: Option<HashMap<String, Value>>,
    }
}

#[post("/create")]
async fn create_task(
    task: web::Json<CreateTaskRequest>,
    app: web::Data<Arc<AppService>>,
) -> Result<HttpResponse, ProcessorError> {
    match app
        .engine_service
        .task
        .create_task(
            CreateTask {
                flow: task.flow.clone(),
                arguments: task.arguments.clone(),
            },
            &app.db_service,
            &app.db_client,
            &app.app,
        )
        .await
    {
        Ok(task) => Ok(HttpResponse::Ok().json(task)),
        Err(err) => Err(ProcessorError::InternalError(err)),
    }
}
