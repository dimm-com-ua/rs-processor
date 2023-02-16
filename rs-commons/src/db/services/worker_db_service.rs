use chrono::{Duration, Utc};
use serde_json::json;
use crate::adapters::db::client::PgClient;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::FlowElementArgument;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::db::models::task_db::TaskVariableDb;
use crate::db::models::task_worker_db::TaskWorkerDb;
use crate::db::repos::worker_repo::WorkerRepo;
use crate::db::services::{App, DbServices};

#[derive(Clone)]
pub struct WorkerDbService {
    repo: WorkerRepo
}

impl WorkerDbService {
    pub fn new() -> Self {
        WorkerDbService {
            repo: WorkerRepo::new()
        }
    }

    pub async fn fetch_workers(&self, count: i64, db: &PgClient) -> Result<Vec<TaskWorkerDb>, ErrorDefinition> {
        let now= Utc::now();
        let lock_key = uuid::Uuid::new_v4();
        let lock_by = now + Duration::seconds(60);
        match self.repo.fetch_workers(lock_key, now, lock_by, count, db).await {
            Ok(workers) => Ok(workers),
            Err(err) => { Err(err) }
        }
    }

    pub async fn process(&self, worker: TaskWorker, db_client: &PgClient, dbs: &DbServices, app: &App) -> Result<(), ErrorDefinition> {
        match db_client.get_connection().await.build_transaction().start().await {
            Ok(tr) => {
                match dbs.process.get_flow_element(worker.element_id, &tr).await {
                    Ok(flow_element) => {
                        if let Some(h) = app.handler(flow_element.handler_type.name.clone()) {
                            let args = match  dbs.tasks.get_task_variables(
                                worker.task_id.clone(),
                                Some(flow_element.id.clone()),
                                &tr
                            ).await {
                                Ok(args) => { Some(args)}
                                Err(_) => { None }
                            };
                            match h.process(worker, dbs, args, &tr).await {
                                Ok(task) => {
                                    Ok(())
                                }
                                Err(err) => {
                                    Err(ErrorDefinition::with_reason("Couldn't process task".to_string(), json!({"error": format!("{:?}", err)})))
                                }
                            }
                        } else {
                            Err(ErrorDefinition::with_reason("Handler not found".to_string(), json!({"error": flow_element.handler_type.name})))
                        }
                    }
                    Err(err) => {
                        Err(ErrorDefinition::from_db(&err))
                    }
                }
            }
            Err(err) => {
                Err(ErrorDefinition::with_reason("Couldn't create transaction".to_string(), json!({"error": format!("{:?}", err)})))
            }
        }
    }
}