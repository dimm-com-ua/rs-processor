use rs_commons::adapters::db::client::PgClient;
use rs_commons::adapters::db::config::DbConfiguration;
use rs_commons::adapters::db::db_migrations::run_migrations;
use rs_commons::config::config::Config;
use rs_commons::db::services::{App, DbServices};
use rs_processor_engine::services::EngineServices;

#[derive(Clone)]
pub struct AppService {
    pub db_client: PgClient,
    pub db_service: DbServices,
    pub engine_service: EngineServices,
    pub app: App
}

#[derive(Debug)]
pub enum AppServiceError {
    ErrorDbInitialization(String),
}

impl AppService {
    pub async fn new(app_config: &Config) -> Result<Self, AppServiceError> {
        Ok(AppService {
            db_client: PgClient::new(app_config.get_db_config()),
            db_service: DbServices::new(),
            engine_service: EngineServices::new(),
            app: App::new()
        })
    }

    pub async fn prepare(&self) {
        run_migrations(&self.db_client).await;
    }
}
