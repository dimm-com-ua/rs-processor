use std::collections::HashMap;
use chrono::{DateTime, Utc};
use deadpool_postgres::Transaction;
use serde_json::Value;
use uuid::Uuid;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::FlowElementArgument;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::db::repos::task_repo::TasksDbRepo;

#[derive(Clone)]
pub struct TasksDbService {
    repo: TasksDbRepo
}

impl TasksDbService {
    pub fn new() -> Self {
        TasksDbService {
            repo: TasksDbRepo::new()
        }
    }

    pub async fn create(&self, flow_uuid: Uuid, current_item_uuid: Uuid, args: Option<HashMap<String, Value>>, element_args: Vec<FlowElementArgument>, tr: &Transaction<'_>) -> Result<Uuid, ErrorDefinition> {
        let created_at= Utc::now();
        self.repo.create_task(flow_uuid, current_item_uuid, created_at, args, element_args, tr).await
    }

    pub async fn create_worker(&self, task_id: Uuid, element_id: Uuid, run_after: Option<DateTime<Utc>>, tr: &Transaction<'_>) -> Result<Uuid, ErrorDefinition> {
        let created_at = Utc::now();
        self.repo.create_worker(task_id,  element_id, created_at, run_after, tr).await
    }

    pub async fn get_task_variables(&self, task_id: Uuid, element_id: Option<Uuid>, tr: &Transaction<'_>) -> Result<Vec<TaskVariable>, ErrorDefinition> {
        self.repo.get_task_variables(task_id, element_id, tr).await
    }
}