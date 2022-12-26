use rs_commons::config::config::Config;
use rs_commons::db::client::{DbError, MongoDbClient};
use rs_commons::db::config::DbConfig;

pub struct AppService {
    pub db_client: MongoDbClient
}

#[derive(Debug)]
pub enum AppServiceError {
    ErrorDbInitialization(String)
}

impl AppService {
    pub async fn new(app_config: Config) -> Result<Self, AppServiceError> {
        let db_client= MongoDbClient::new(app_config.get_db_config()).await.map_err(|err| {
            AppServiceError::ErrorDbInitialization(format!("{:?}", err))
        }).unwrap();

        Ok(AppService {
            db_client
        })
    }
}