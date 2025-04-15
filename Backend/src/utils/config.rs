use std::env;

#[derive(Debug)]
pub struct Config {
    pub port: u16,
}

impl Config {
    pub fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        let port = env::var("PORT")
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(3000)) // Default to 3000 if PORT is unset
            .map_err(|e| format!("Invalid PORT value: {}", e))?;

        Ok(Config { port })
    }
}
