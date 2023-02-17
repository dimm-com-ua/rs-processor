use deadpool_postgres::Transaction;
use crate::adapters::models::common_error::ErrorDefinition;
use crate::db::models::data_type_db::DataTypeDb;
use crate::db::repos::core_repo::CoreRepo;

#[derive(Clone)]
pub struct CoreDbService {
    repo: CoreRepo
}

impl CoreDbService {
    pub fn new() -> Self {
        CoreDbService {
            repo: CoreRepo::new()
        }
    }

    pub async fn get_data_type(&self, dt_name: String, tr: &Transaction<'_>) -> Result<DataTypeDb, ErrorDefinition> {
        self.repo.get_data_type(dt_name, tr).await
    }
}