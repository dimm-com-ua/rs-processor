use crate::config::config::Config;

pub struct DbConfig {
    pub db_path: String,
    pub db_username: String,
    pub db_password: String,
    pub db_database: String
}

pub trait DbConfiguration {
    fn get_db_config(&self) -> DbConfig;
}

impl Config {
    pub fn get_db_config(&self) -> DbConfig {
        DbConfig {
            db_path: self.mongodb_url.clone(),
            db_username: self.mongodb_username.clone(),
            db_password: self.mongodb_password.clone(),
            db_database: self.mongodb_database.clone()
        }
    }
}