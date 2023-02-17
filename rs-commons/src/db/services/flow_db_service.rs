use deadpool_postgres::Transaction;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::adapters::models::process::flow_element::FlowElementArgument;
use crate::adapters::models::worker::task_variable::TaskVariable;
use crate::db::models::data_type_db::DataTypeDb;
use crate::db::models::flow_db::FlowElementArgumentDb;
use crate::db::repos::flow_repo::FlowRepo;
use crate::db::services::DbServiceError;

#[derive(Clone)]
pub struct FlowDbService {
    repo: FlowRepo
}

impl FlowDbService {
    pub fn new() -> Self {
        FlowDbService { repo: FlowRepo::new() }
    }

    pub async fn get_flow_item_arguments(&self, element_id: uuid::Uuid, tr: &Transaction<'_>) -> Result<Vec<FlowElementArgument>, DbServiceError> {
        match self.repo.get_flow_item_arguments(element_id, tr).await {
            Ok(args) => {
                Ok(args.iter().map(|(arg, dt)| { FlowElementArgument::from_db(arg, dt) }).collect())
            }
            Err(err) => {
                Err(DbServiceError::QueryError(format!("{:?}", err)))
            }
        }
    }

    pub async fn save_flow_item_out_variables(&self, task_id: uuid::Uuid, element_id: uuid::Uuid, arg_names: Vec<String>, task_variables: Vec<TaskVariable>, tr: &Transaction<'_>) -> Result<Vec<(FlowElementArgumentDb, DataTypeDb)>, ErrorDefinition> {
        self.repo.save_flow_item_out_variables(task_id, element_id, arg_names, task_variables, tr).await
    }
}