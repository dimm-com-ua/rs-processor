use chrono::{Duration, Utc};
use deadpool_postgres::tokio_postgres::Error;
use deadpool_postgres::Transaction;
use log::info;
use serde_json::json;
use crate::adapters::db::client::PgClient;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::{ArgumentDirection, FlowElementArgument};
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::{TaskWorker, WorkerWhat};
use crate::adapters::models::worker::task_worker_result::WorkerResult;
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

    pub async fn fetch_workers(&self, count: i64, db: &PgClient) -> Result<Vec<TaskWorker>, ErrorDefinition> {
        let now= Utc::now();
        let lock_key = uuid::Uuid::new_v4();
        let lock_by = now + Duration::seconds(60);
        match self.repo.fetch_workers(lock_key, now, lock_by, count, db).await {
            Ok(workers) =>  {
                Ok(
                    workers.iter().map(|w| {
                        TaskWorker::from_db(w)
                    }).collect()
                )
            }
            Err(err) => { Err(err) }
        }
    }

    pub async fn process(&self, worker: TaskWorker, db_client: &PgClient, dbs: &DbServices, app: &App) -> Result<(), ErrorDefinition> {
        match db_client.get_connection().await.build_transaction().start().await {
            Ok(tr) => {
                match dbs.process.get_flow_element(worker.element_id, &tr).await {
                    Ok(flow_element) => {
                        info!("This is {}", flow_element.description.clone());
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
                            if let Err(err) = dbs.tasks.set_current_flow_item(task_id.clone(), flow_element.id.clone(), &tr).await {
                                return Err(ErrorDefinition::with_reason("Couldn't set current element for task".to_string(), json!({"error": format!("{:?}", err)})))
                            }
                            match h.process(worker, dbs, app, args, &tr).await {
                                Ok(task) => {
                                    match task.result {
                                        WorkerResult::Done => {
                                            match dbs.flow.get_flow_item_arguments(flow_element.id.clone(), &tr).await {
                                                Ok(args_to_save) => {
                                                    let arg_names: Vec<String> = args_to_save.iter()
                                                        .filter(|x| x.direction == ArgumentDirection::Out)
                                                        .map(|x| x.name.clone()).collect();
                                                    if arg_names.is_empty() == false {
                                                        match dbs.flow.save_flow_item_out_variables(
                                                            task_id.clone(),
                                                            flow_element.id.clone(),
                                                            arg_names,
                                                            task.out_args,
                                                            &tr
                                                        ).await {
                                                            Ok(res) => {}
                                                            Err(err) => {
                                                                return Err(ErrorDefinition::with_reason("Couldn't save process task result".to_string(), json!({"error": format!("{:?}", err)})))
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(_) => {}
                                            }
                                            if let Err(err) = self.repo.delete(worker_id, &tr).await {
                                                return Err(ErrorDefinition::with_reason("Couldn't deleting worker".to_string(), json!({"error": format!("{:?}", err)})))
                                            }
                                            let now = Utc::now();
                                            let _ = dbs.tasks.create_worker(task_id.clone(),  flow_element.id.clone(),WorkerWhat::RouteAfter, Some(now), &tr).await;
                                        }
                                        WorkerResult::Fail => {}
                                        WorkerResult::Finishing => {}
                                    }
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

    pub async fn route_after(&self, worker: TaskWorker, db_client: &PgClient, dbs: &DbServices, app: &App) -> Result<(), ErrorDefinition> {
        match db_client.get_connection().await.build_transaction().start().await {
            Ok(tr) => {
                match dbs.process.get_out_routes(
                    worker.element_id.clone(),
                    &tr
                ).await {
                    Ok(routes) => {
                        if let Some(route) = routes.first() {
                            if let Some(is_conditional) = route.model.is_conditional {
                                match dbs.tasks.create_worker(
                                    worker.task_id.clone(),
                                    route.in_flow_element.as_ref().unwrap().id.clone(),
                                    WorkerWhat::Process,
                                    Some(Utc::now()),
                                    &tr
                                ).await {
                                    Ok(_) => {}
                                    Err(err) => {
                                        return Err(err);
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        return Err(ErrorDefinition::with_reason("Couldn't get out routes".to_string(), json!({"error": format!("{:?}", err)})))
                    }
                }
                if let Err(err) = self.repo.delete(worker.id.clone(), &tr).await {
                    return Err(ErrorDefinition::with_reason("Couldn't deleting worker".to_string(), json!({"error": format!("{:?}", err)})))
                }
                let _ = tr.commit().await;
                return Ok(());
            }
            Err(err) => {
                Err(ErrorDefinition::with_reason("Couldn't create transaction".to_string(), json!({"error": format!("{:?}", err)})))
            }
        }
    }
}