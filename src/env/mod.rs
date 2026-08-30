use std::env;

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub env: String,
    pub github_token: String,
}

impl Config {
    pub fn get_env() -> Config {
        let host = env::var("HOST").unwrap_or("http://192.0.0.1".to_string());
        let port: u16 = match env::var("port") {
          Ok(val) => val.parse().unwrap_or(8080),
          Err(_) => 8080,
        };
        let env = env::var("ENV").unwrap_or("dev".to_string());
        let github_token = env::var("GITHUB_TOKEN").unwrap_or("".to_string());

        Config {
            host,
            port,
            env,
            github_token
        }
    }
}

