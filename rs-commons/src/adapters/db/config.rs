use deadpool_postgres::SslMode;
use crate::config::config::Config;

pub struct DbConfig {
    pub db_path: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_database: String,
    pub db_ssl_mode: SslMode
}

pub trait DbConfiguration {
    fn get_db_config(&self) -> DbConfig;
}

impl DbConfiguration for Config {
    fn get_db_config(&self) -> DbConfig {
        DbConfig {
            db_path: self.pg_url.clone(),
            db_port: self.pg_port.clone(),
            db_username: self.pg_username.clone(),
            db_password: self.pg_password.clone(),
            db_database: self.pg_database.clone(),
            db_ssl_mode: match self.pg_database_use_tls {
                true => { SslMode::Require }
                false => { SslMode::Disable }
            }
        }
    }
}
