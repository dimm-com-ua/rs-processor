use chrono::{Duration, Utc};
use log::info;
use serde_json::json;

use crate::adapters::db::client::PgClient;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::ArgumentDirection;
use crate::adapters::models::process::flow_route::FlowRoute;
use crate::adapters::models::worker::task_worker::{TaskWorker, WorkerWhat};
use crate::adapters::models::worker::task_worker_result::WorkerResult;
use crate::db::repos::worker_repo::WorkerRepo;
use crate::db::services::{App, DbServices};

#[derive(Clone)]
pub struct WorkerDbService {
    repo: WorkerRepo,
}

impl WorkerDbService {
    pub fn new() -> Self {
        WorkerDbService {
            repo: WorkerRepo::new(),
        }
    }

    pub async fn get_worker(
        &self,
        uuid: uuid::Uuid,
        db: &PgClient,
    ) -> Result<TaskWorker, ErrorDefinition> {
        self.repo.get_worker(uuid, db).await
    }

    pub async fn fetch_workers(
        &self,
        count: i64,
        db: &PgClient,
    ) -> Result<Vec<TaskWorker>, ErrorDefinition> {
        let now = Utc::now();
        let lock_key = uuid::Uuid::new_v4();
        let lock_by = now + Duration::seconds(60);
        match self
            .repo
            .fetch_workers(lock_key, now, lock_by, count, db)
            .await
        {
            Ok(workers) => Ok(workers.iter().map(|w| TaskWorker::from_db(w)).collect()),
            Err(err) => Err(err),
        }
    }

    pub async fn process(
        &self,
        worker: TaskWorker,
        db_client: &PgClient,
        dbs: &DbServices,
        app: &App,
    ) -> Result<(), ErrorDefinition> {
        match db_client
            .get_connection()
            .await
            .build_transaction()
            .start()
            .await
        {
            Ok(tr) => {
                match dbs.process.get_flow_element(worker.element_id, &tr).await {
                    Ok(flow_element) => {
                        if let Some(h) = app.handler(flow_element.handler_type.name.clone()) {
                            let args = match dbs
                                .tasks
                                .get_task_variables(worker.task_id.clone(), None, &tr)
                                .await
                            {
                                Ok(args) => Some(args),
                                Err(err) => {
                                    return Err(err);
                                }
                            };
                            let task_id = worker.task_id.clone();
                            let worker_id = worker.id.clone();
                            if let Err(err) = dbs
                                .tasks
                                .set_current_flow_item(
                                    task_id.clone(),
                                    flow_element.id.clone(),
                                    &tr,
                                )
                                .await
                            {
                                return Err(ErrorDefinition::with_reason(
                                    "Couldn't set current element for task".to_string(),
                                    json!({ "error": format!("{:?}", err) }),
                                ));
                            }
                            match h
                                .lock()
                                .await
                                .process(worker, &flow_element, dbs, app, args, &tr)
                                .await
                            {
                                Ok(task) => {
                                    match task.result {
                                        WorkerResult::Done => {
                                            match dbs
                                                .flow
                                                .get_flow_item_arguments(
                                                    flow_element.id.clone(),
                                                    &tr,
                                                )
                                                .await
                                            {
                                                Ok(args_to_save) => {
                                                    let arg_names: Vec<String> = args_to_save
                                                        .iter()
                                                        .filter(|x| {
                                                            x.direction == ArgumentDirection::Out
                                                        })
                                                        .map(|x| x.name.clone())
                                                        .collect();
                                                    if arg_names.is_empty() == false {
                                                        match dbs.flow.save_flow_item_out_variables(
                                                            task_id.clone(),
                                                            flow_element.id.clone(),
                                                            arg_names,
                                                            task.out_args,
                                                            &tr
                                                        ).await {
                                                            Ok(_res) => {}
                                                            Err(err) => {
                                                                return Err(ErrorDefinition::with_reason("Couldn't save process task result".to_string(), json!({"error": format!("{:?}", err)})))
                                                            }
                                                        }
                                                    }
                                                }
                                                Err(_) => {}
                                            }
                                            if let Err(err) = self.repo.delete(worker_id, &tr).await
                                            {
                                                return Err(ErrorDefinition::with_reason(
                                                    "Couldn't deleting worker".to_string(),
                                                    json!({ "error": format!("{:?}", err) }),
                                                ));
                                            }
                                            let now = Utc::now();
                                            let _ = dbs
                                                .tasks
                                                .create_worker(
                                                    task_id.clone(),
                                                    flow_element.id.clone(),
                                                    WorkerWhat::RouteAfter,
                                                    Some(now),
                                                    app,
                                                    &tr,
                                                )
                                                .await;
                                        }
                                        WorkerResult::Fail => {}
                                        WorkerResult::Finishing => {}
                                    }
                                    match tr.commit().await {
                                        Ok(_) => Ok(()),
                                        Err(err) => {
                                            return Err(ErrorDefinition::with_reason(
                                                "Couldn't commit transaction".to_string(),
                                                json!({ "error": format!("{:?}", err) }),
                                            ))
                                        }
                                    }
                                }
                                Err(err) => Err(ErrorDefinition::with_reason(
                                    "Couldn't process task".to_string(),
                                    json!({ "error": format!("{:?}", err) }),
                                )),
                            }
                        } else {
                            Err(ErrorDefinition::with_reason(
                                "Handler not found".to_string(),
                                json!({"error": flow_element.handler_type.name}),
                            ))
                        }
                    }
                    Err(err) => Err(ErrorDefinition::from_db(&err)),
                }
            }
            Err(err) => Err(ErrorDefinition::with_reason(
                "Couldn't create transaction".to_string(),
                json!({ "error": format!("{:?}", err) }),
            )),
        }
    }

    pub async fn route_after(
        &self,
        worker: TaskWorker,
        db_client: &PgClient,
        dbs: &DbServices,
        app: &App,
    ) -> Result<(), ErrorDefinition> {
        match db_client
            .get_connection()
            .await
            .build_transaction()
            .start()
            .await
        {
            Ok(tr) => {
                match dbs
                    .process
                    .get_out_routes(worker.element_id.clone(), &tr)
                    .await
                {
                    Ok(routes) => {
                        let mut route: Option<FlowRoute> = None;
                        for rt in routes {
                            if let Some(is_conditional) = rt.model.is_conditional {
                                if is_conditional == false {
                                    route = Some(rt);
                                    break;
                                } else {
                                    match rt.model.condition.clone() {
                                        None => {}
                                        Some(cond) => {
                                            info!("{}", cond);
                                            if let Some(cond) = cond.get("if") {
                                                match app
                                                    .js_code
                                                    .evaluate_from_task::<bool>(
                                                        worker.task_id.clone(),
                                                        cond.as_str().unwrap().to_string(),
                                                        dbs,
                                                        &tr,
                                                    )
                                                    .await
                                                {
                                                    Ok(res) => {
                                                        info!("res: {}", res);
                                                        if res == true {
                                                            route = Some(rt);
                                                            break;
                                                        }
                                                    }
                                                    Err(err) => {
                                                        return Err(ErrorDefinition::with_reason(
                                                            "Couldn't execute script".to_string(),
                                                            json!({
                                                                "error": format!("{:?}", err)
                                                            }),
                                                        ))
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        if let Some(route) = route {
                            return match dbs
                                .tasks
                                .create_worker(
                                    worker.task_id.clone(),
                                    route.in_flow_element.as_ref().unwrap().id.clone(),
                                    WorkerWhat::Process,
                                    Some(Utc::now()),
                                    app,
                                    &tr,
                                )
                                .await
                            {
                                Ok(_) => {
                                    if let Err(err) = self.repo.delete(worker.id.clone(), &tr).await
                                    {
                                        return Err(ErrorDefinition::with_reason(
                                            "Couldn't deleting worker".to_string(),
                                            json!({ "error": format!("{:?}", err) }),
                                        ));
                                    }
                                    let _ = tr.commit().await;
                                    Ok(())
                                }
                                Err(err) => Err(err),
                            };
                        } else {
                            Err(ErrorDefinition::empty(
                                "No route found to be run".to_string(),
                            ))
                        }
                    }
                    Err(err) => {
                        return Err(ErrorDefinition::with_reason(
                            "Couldn't get out routes".to_string(),
                            json!({ "error": format!("{:?}", err) }),
                        ))
                    }
                }
            }
            Err(err) => Err(ErrorDefinition::with_reason(
                "Couldn't create transaction".to_string(),
                json!({ "error": format!("{:?}", err) }),
            )),
        }
    }
}
