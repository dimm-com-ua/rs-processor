use rs_commons::adapters::db::client::PgClient;
use rs_commons::adapters::models::process::ProcessError;
use rs_commons::adapters::models::task::{CreateTask, TaskDefinition};
use rs_commons::db::services::{App, DbServices};

#[derive(Clone)]
pub struct TaskEngineService;

#[derive(Debug)]
pub enum FlowElementError {
    ElementDoesNotMeetRequiredArguments
}

#[derive(Debug)]
pub enum TaskEngineError {
    DbServiceError(String),
    NotFound(String),
    FlowDoesNotContainsStartingElement,
    FlowElementError(FlowElementError)
}

impl TaskEngineService {
    pub fn new() -> Self { TaskEngineService{} }

    pub async fn create_task(&self, task: CreateTask, dbs: &DbServices, db_client: &PgClient, app: &App) -> Result<TaskDefinition, TaskEngineError> {
        match db_client.get_connection().await
            .build_transaction().start().await {
            Ok(tr) => {
                match dbs.process.find_process_flow(task.flow, &tr).await {
                    Ok(flow) => {
                        log::info!("{:?}", flow);
                        match dbs.process.find_starting_element(flow.id, &tr).await {
                            Ok(starting_element) => {
                                match starting_element.process(task.arguments, dbs, &tr, &app).await {
                                    Ok(_) => {}
                                    Err(err) => {
                                        return Err(TaskEngineError::DbServiceError(format!("{:?}", err)))
                                    }
                                }

                                if let Ok(args) = starting_element.get_all_arguments(dbs, &tr).await {
                                    log::info!("args: {:?}", args);
                                } else {
                                    log::info!("args not found");
                                }
                                Ok(TaskDefinition{})
                            }
                            Err(err) => {
                                Err(TaskEngineError::DbServiceError(format!("{:?}", err)))
                            }
                        }
                    }
                    Err(err) => {
                        Err(TaskEngineError::NotFound(format!("Flow not found: {:?}", err)))
                    }
                }
            }
            Err(err) => { Err(TaskEngineError::DbServiceError(err.to_string())) }
        }
    }
}