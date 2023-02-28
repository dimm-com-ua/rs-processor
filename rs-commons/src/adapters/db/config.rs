use crate::adapters::queue_publisher::QueueConfig;
use crate::config::config::Config;
use deadpool_postgres::SslMode;

#[derive(Clone)]
pub struct DbConfig {
    pub db_path: String,
    pub db_port: String,
    pub db_username: String,
    pub db_password: String,
    pub db_database: String,
    pub db_ssl_mode: SslMode,
}

pub trait DbConfiguration {
    fn get_db_config(&self) -> DbConfig;
    fn get_queue_config(&self) -> QueueConfig;
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
                true => SslMode::Require,
                false => SslMode::Disable,
            },
        }
    }

    fn get_queue_config(&self) -> QueueConfig {
        QueueConfig {
            amqp_path: self.queue_amqp_path.clone(),
            queue_name: self.queue_exchange_name.clone(),
        }
    }
}
