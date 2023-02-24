use log::error;
use rs_commons::adapters::models::common_error::ErrorDefinition;
use rs_commons::adapters::models::worker::task_worker::{TaskWorker, WorkerWhat};
use rs_commons::db::services::worker_db_service::WorkerDbService;
use crate::app::app_service::AppService;

pub struct WorkerService {}

impl WorkerService {
    pub fn new() -> Self {
        WorkerService {}
    }

    pub async fn process(&self, w: TaskWorker, app: &AppService) -> Result<(), ErrorDefinition> {
        let app = app.clone();
        let db_client = app.db_client.clone();
        let dbs = app.db_service.clone();
        let db_service = app.db_service.clone();
        match w.what {
            WorkerWhat::Process => {
                actix_web::rt::spawn(async move {
                    db_service.worker.process(w, &db_client, &dbs, &app.app).await
                });
                Ok(())
            }
            WorkerWhat::RouteAfter => {
                actix_web::rt::spawn(async move {
                    if let Err(err) =
                        db_service.worker.route_after(w, &db_client, &dbs, &app.app).await
                    {
                        error!("{:?}", err);
                    }
                });
                Ok(())
            }
        }
    }
}