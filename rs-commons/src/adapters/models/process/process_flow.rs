use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::FlowElement;
use crate::adapters::models::task::TaskDefinition;
use crate::db::services::{App, DbServices};
use deadpool_postgres::Transaction;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ProcessFlow {
    pub id: uuid::Uuid,
    pub name: String,
    pub enabled: bool,
    pub version_id: i32,
}

impl ProcessFlow {
    pub async fn run_task(
        &self,
        starting_element: &FlowElement,
        args_to_process: Option<HashMap<String, Value>>,
        dbs: &DbServices,
        tr: &Transaction<'_>,
        _app: &App,
    ) -> Result<TaskDefinition, ErrorDefinition> {
        TaskDefinition::create(self.id, starting_element, args_to_process, dbs, tr).await
    }
}
