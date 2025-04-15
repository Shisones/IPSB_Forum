use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub port: u16,
    pub database_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self, std::env::VarError> {
        dotenvy::dotenv().ok();
        Ok(Config {
            port: std::env::var("PORT")
                .unwrap_or_else(|_| "3000".to_string())
                .parse()
                .unwrap(),
            database_url: std::env::var("DATABASE_URL")?,
        })
    }
}
