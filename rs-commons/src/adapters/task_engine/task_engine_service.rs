use chrono::Utc;
use serde_json::{json};
use crate::adapters::db::client::PgClient;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::task::{CreateTask, TaskDefinition};
use crate::adapters::models::worker::task_worker::WorkerWhat;
use crate::db::services::{App, DbServices};

#[derive(Clone)]
pub struct TaskEngineService;

#[derive(Debug)]
pub enum FlowElementError {
    ElementDoesNotMeetRequiredArguments
}

#[derive(Debug)]
pub enum TaskEngineError {
    DbServiceError(ErrorDefinition),
    NotFound(ErrorDefinition),
    FlowDoesNotContainsStartingElement,
    FlowElementError(FlowElementError)
}

impl TaskEngineService {
    pub fn new() -> Self { TaskEngineService{} }

    pub async fn create_task(&self, task: CreateTask, dbs: &DbServices, db_client: &PgClient, app: &App) -> Result<TaskDefinition, ErrorDefinition> {
        match db_client.get_connection().await
            .build_transaction().start().await {
            Ok(tr) => {
                match dbs.process.find_process_flow(task.flow, &tr).await {
                    Ok(flow) => {
                        match dbs.process.find_starting_element(flow.id, &tr).await {
                            Ok(starting_element) => {
                                if let Err(err) = starting_element.validate(task.arguments.clone(), dbs, &tr, &app).await {
                                    return Err(err)
                                }
                                match flow.run_task(&starting_element, task.arguments, dbs, &tr, &app).await {
                                    Ok(task) => {
                                        let now = Utc::now();
                                        if let Err(err) = dbs.tasks.create_worker(task.id,  starting_element.id.clone(),WorkerWhat::Process, Some(now), &tr).await {
                                            return Err(ErrorDefinition::with_reason("Error creating worker".to_string(), json!({"error": format!("{:?}", err)})))
                                        }

                                        if let Err(err) = tr.commit().await {
                                            Err(ErrorDefinition::with_reason("Error committing transaction".to_string(), json!({"error": format!("{:?}", err)})))
                                        } else {
                                            Ok(task)
                                        }
                                    }
                                    Err(err) => {
                                        Err(err)
                                    }
                                }
                            }
                            Err(err) => {
                                Err(ErrorDefinition::from_db(&err))
                            }
                        }
                    }
                    Err(err) => {
                        Err(ErrorDefinition::from_db(&err))
                    }
                }
            }
            Err(err) => {
                Err(ErrorDefinition::empty(format!("{:?}", err)))
            }
        }
    }

}