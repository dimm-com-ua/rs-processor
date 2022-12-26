use mongodb::{Client, Database};
use mongodb::options::ClientOptions;
use crate::db::config::DbConfig;

pub struct MongoDbClient {
    pub database: Database
}

#[derive(Debug)]
pub enum DbError {
    ConnectError(String)
}

impl MongoDbClient {
    pub async fn new(db_config: DbConfig) -> Result<MongoDbClient, DbError> {
        match ClientOptions::parse(format!("mongodb+srv://{}:{}@{}/?retryWrites=true&w=majority", db_config.db_username, db_config.db_password, db_config.db_path))
            .await {
            Ok(client_options) => match Client::with_options(client_options) {
                Ok(client) => {
                    Ok(MongoDbClient { database: client.database(db_config.db_database.as_str()) })
                }
                Err(err) => { Err(DbError::ConnectError(format!("{:?}", err))) }
            },
            Err(err) => { Err(DbError::ConnectError(format!("{:?}", err))) }
        }
    }
}