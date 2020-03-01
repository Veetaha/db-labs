use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub db_host: String,
    pub db_user: String,
    pub db_port: u16,
    pub db_password: String,
    pub db_db: String,
    pub db_connect_timeout: u64,
    pub database_url: String
}
