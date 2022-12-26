use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub mongodb_url: String,
    pub mongodb_username: String,
    pub mongodb_password: String,
    pub mongodb_database: String
}

impl Config {
    pub fn make_from_env() -> Self {
        let _ = dotenv::dotenv();
        envy::from_env::<Config>().expect("Couldn't read required value")
    }
}