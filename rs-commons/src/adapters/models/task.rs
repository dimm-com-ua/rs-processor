use std::collections::HashMap;
use deadpool_postgres::Transaction;
use serde_json::Value;
use serde::{Serialize};
use uuid::Uuid;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::FlowElement;
use crate::db::services::DbServices;

pub struct CreateTask {
    pub flow: String,
    pub arguments: Option<HashMap<String, Value>>
}

#[derive(Serialize)]
pub struct TaskDefinition {
    pub id: uuid::Uuid,
    pub current_element: uuid::Uuid
}

impl TaskDefinition {
    pub async fn create(flow_uuid: Uuid, starting_element: &FlowElement, args_to_process: Option<HashMap<String, Value>>, dbs: &DbServices, tr: &Transaction<'_>) -> Result<Self, ErrorDefinition> {
        let element_args = dbs.flow.get_flow_item_arguments(starting_element.id.clone(), tr).await.unwrap_or(Vec::new());
        match dbs.tasks.create(flow_uuid, starting_element.id,args_to_process, element_args, tr).await {
            Ok(uuid) => {
                Ok(TaskDefinition {
                    id: uuid,
                    current_element: starting_element.id,
                })
            }
            Err(err) => { Err(err) }
        }
    }
}