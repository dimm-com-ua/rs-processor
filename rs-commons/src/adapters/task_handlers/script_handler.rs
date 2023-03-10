use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::FlowElement;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::adapters::models::worker::task_worker::TaskWorker;
use crate::adapters::models::worker::task_worker_result::TaskWorkerResult;
use crate::adapters::task_handlers::TaskHandlerTrait;
use crate::db::services::{App, DbServices};
use async_trait::async_trait;
use deadpool_postgres::Transaction;
use std::sync::Arc;
use log::info;

pub struct ScriptHandler;

impl ScriptHandler {
    pub fn new() -> Self {
        ScriptHandler {}
    }
}

#[async_trait]
impl TaskHandlerTrait for ScriptHandler {
    async fn process(
        &self,
        task_worker: TaskWorker,
        flow_element: &FlowElement,
        dbs: &DbServices,
        app: &App,
        _args: Option<Vec<TaskVariable>>,
        tr: &Transaction<'_>,
    ) -> Result<TaskWorkerResult, ErrorDefinition> {
        if let Some(script) = flow_element.handler_value.get("script") {
            return match app
                .js_code()
                .evaluate_from_flow_element(
                    script.as_str().unwrap(),
                    &task_worker,
                    Arc::new(dbs),
                    Arc::new(tr),
                )
                .await {
                Ok(vars) => {
                    Ok(TaskWorkerResult::ok_with_args(vars))
                }
                Err(err) => {
                    info!("{:?}", err);
                    Err(err)
                }
            };
        } else {
            Err(ErrorDefinition::empty(
                "Flow element does not contains script as property".to_string(),
            ))
        }
    }
}
