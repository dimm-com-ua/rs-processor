use deadpool_postgres::Transaction;
use crate::adapters::models::process::{flow_element::FlowElement, process_flow::ProcessFlow};
use crate::db::repos::process_repo::ProcessRepo;
use crate::db::services::{DbServiceError};

#[derive(Clone)]
pub struct ProcessDbService {
    repo: ProcessRepo
}


impl ProcessDbService {
    pub fn new() -> Self {
        ProcessDbService { repo: ProcessRepo::new() }
    }

    pub async fn find_process_flow(&self, flow_code: String, tr: &Transaction<'_>) -> Result<ProcessFlow, DbServiceError> {
        match self.repo.find_flow(flow_code.as_str(), tr).await {
            Ok(process_flow) => {
                Ok(process_flow)
            },
            Err(err) => Err(DbServiceError::QueryError(format!("{:?}", err)))
        }
    }

    pub async fn find_starting_element(&self, flow_id: uuid::Uuid, tr: &Transaction<'_>) -> Result<FlowElement, DbServiceError> {
        match self.repo.find_starting_element(flow_id, tr).await {
            Ok(flow_element) => Ok(flow_element),
            Err(err) => {
                Err(DbServiceError::QueryError(format!("{:?}", err)))
            }
        }
    }

    pub async fn get_flow_element(&self, element_id: uuid::Uuid, tr: &Transaction<'_>) -> Result<FlowElement, DbServiceError> {
        match self.repo.get_flow_element(element_id, tr).await {
            Ok(flow_element) => Ok(flow_element),
            Err(err) => {
                Err(DbServiceError::QueryError(format!("{:?}", err)))
            }
        }
    }
}