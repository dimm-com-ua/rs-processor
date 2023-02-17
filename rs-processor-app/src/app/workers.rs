use std::sync::Arc;
use log::error;
use rs_commons::adapters::models::common_error::ErrorDefinition;
use rs_commons::adapters::models::worker::task_worker::TaskWorker;
use rs_commons::db::services::worker_db_service::WorkerDbService;
use crate::app::app_service::AppService;

pub struct WorkerService {
    db_service: WorkerDbService
}

impl WorkerService {
    pub fn new() -> Self {
        WorkerService {
            db_service: WorkerDbService::new()
        }
    }

    pub async fn process_workers(&self, app: Arc<AppService>) -> Result<(), ErrorDefinition> {
        match app.db_service.worker.fetch_workers(10, &app.db_client).await {
            Ok(workers) => {
                for w in workers {
                    let app = app.clone();
                    let db_client = app.db_client.clone();
                    let dbs = app.db_service.clone();
                    let db_service = self.db_service.clone();
                    tokio::spawn(async move {
                        if let Err(err) = db_service.process(TaskWorker::from_db(w), &db_client, &dbs, &app.app).await {
                            error!("{:?}", err);
                        }
                    });
                }
                Ok(())
            }
            Err(err) => { Err(err) }
        }
    }
}
