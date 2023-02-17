use chrono::{Duration, Utc};
use deadpool_postgres::tokio_postgres::Error;
use serde_json::json;
use crate::adapters::db::client::PgClient;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::{ArgumentDirection, FlowElementArgument};
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::db::models::data_type_db::DataTypeDb;
use crate::db::models::flow_db::FlowElementArgumentDb;
use crate::db::models::task_db::TaskVariableDb;
use crate::db::models::task_worker_db::TaskWorkerDb;
use crate::db::repos::worker_repo::WorkerRepo;
use crate::db::services::{App, DbServiceError, DbServices};

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
                                None,
                                &tr
                            ).await {
                                Ok(args) => { Some(args)}
                                Err(err) => {
                                    return Err(err);
                                }
                            };
                            let task_id = worker.task_id.clone();
                            let worker_id = worker.id.clone();
                            match h.process(worker, dbs, args, &tr).await {
                                Ok(task) => {
                                    match dbs.flow.get_flow_item_arguments(flow_element.id.clone(), &tr).await {
                                        Ok(args_to_save) => {
                                            let arg_names: Vec<String> = args_to_save.iter()
                                                .filter(|x| x.direction == ArgumentDirection::Out)
                                                .map(|x| x.name.clone()).collect();
                                            match dbs.flow.save_flow_item_out_variables(
                                                task_id.clone(),
                                                flow_element.id.clone(),
                                                arg_names,
                                                task.out_args,
                                                &tr
                                            ).await {
                                                Ok(res) => {  }
                                                Err(err) => {
                                                    return Err(ErrorDefinition::with_reason("Couldn't save process task result".to_string(), json!({"error": format!("{:?}", err)})))
                                                }
                                            }
                                        }
                                        Err(_) => {}
                                    }
                                    let _ = self.repo.delete(worker_id, &tr).await;
                                    match tr.commit().await {
                                        Ok(_) => { Ok(()) }
                                        Err(err) => {
                                            return Err(ErrorDefinition::with_reason("Couldn't commit transaction".to_string(), json!({"error": format!("{:?}", err)})))
                                        }
                                    }
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