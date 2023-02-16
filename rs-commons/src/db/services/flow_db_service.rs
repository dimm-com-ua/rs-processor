use deadpool_postgres::Transaction;
use crate::adapters::models::process::flow_element::FlowElementArgument;
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
}