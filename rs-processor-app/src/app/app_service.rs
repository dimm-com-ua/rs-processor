use rs_commons::adapters::db::client::PgClient;
use rs_commons::adapters::db::config::DbConfiguration;
use rs_commons::adapters::db::db_migrations::run_migrations;
use rs_commons::adapters::models::common_error::ErrorDefinition;
use rs_commons::adapters::task_engine::EngineServices;
use rs_commons::config::config::Config;
use rs_commons::db::services::{App, DbServices};

#[derive(Clone)]
pub struct AppService {
    pub db_client: PgClient,
    pub db_service: DbServices,
    pub engine_service: EngineServices,
    pub app: App,
}

impl AppService {
    pub async fn new(app_config: &Config) -> Result<Self, ErrorDefinition> {
        match App::new(app_config).await {
            Ok(app) => Ok(AppService {
                db_client: PgClient::new(app_config.get_db_config()),
                db_service: DbServices::new(),
                engine_service: EngineServices::new(),
                app,
            }),
            Err(err) => Err(err),
        }
    }

    pub async fn prepare(&self) {
        run_migrations(&self.db_client).await;
    }
}
