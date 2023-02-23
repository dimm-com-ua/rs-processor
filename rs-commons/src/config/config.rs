use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub pg_url: String,
    pub pg_port: String,
    pub pg_username: String,
    pub pg_password: String,
    pub pg_database: String,
    pub pg_database_use_tls: bool,

    pub app_port: u16,
    pub app_host: String,
}

impl Config {
    pub fn make_from_env() -> Self {
        let _ = dotenv::dotenv();
        envy::from_env::<Config>().expect("Couldn't read required value")
    }
}
