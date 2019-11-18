use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub database_url: String
}
