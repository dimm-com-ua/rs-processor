use deadpool_postgres::{Client, Config, Pool, SslMode};
use deadpool_postgres::tokio_postgres::{NoTls};
use openssl::ssl::{SslConnector, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use crate::adapters::db::config::DbConfig;

#[derive(Clone)]
pub struct PgClient {
    pg_config: Config,
    pg_pool: Option<Pool>
}

impl PgClient {
    pub fn new(pg_config: DbConfig) -> Self {
        let mut config = Config::new();

        config.dbname = Some(pg_config.db_database);
        config.host = Some(pg_config.db_path);
        config.port = Some(pg_config.db_port.parse().unwrap());
        config.user = Some(pg_config.db_username);
        config.password = Some(pg_config.db_password);
        config.ssl_mode = Some(pg_config.db_ssl_mode);

        let mut pg_client = PgClient {
            pg_config: config,
            pg_pool: None
        };

        pg_client.pg_pool = Some(pg_client.create_pool());
        pg_client
    }

    pub fn create_pool(&self) -> Pool {
        match self.pg_config.ssl_mode.unwrap() {
            SslMode::Disable => {
                match self.pg_config.create_pool(None, NoTls) {
                    Ok(p) => p,
                    Err(err) => { panic!("{}", err.to_string()) }
                }
            }
            _ => {
                let mut builder = SslConnector::builder(SslMethod::tls()).unwrap();
                builder.set_verify(SslVerifyMode::NONE);
                let connector = MakeTlsConnector::new(builder.build());

                match self.pg_config.create_pool(None, connector) {
                    Ok(p) => p,
                    Err(err) => panic!("{}", err.to_string())
                }
            }
        }
    }

    pub async fn get_connection(&self) -> Client {
        match &self.pg_pool {
            None => panic!("Couldn't init connection pool!"),
            Some(pool) => { pool.get().await.expect("Couldn't get connection from the pool!") }
        }
    }
}