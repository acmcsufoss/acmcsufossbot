use std::env;

pub struct Config {
    pub host: String,
    pub port: String,
    pub env: String,
    pub github_token: String
}

impl Config {
    pub fn get_env() -> Config{
        let host = env::var("HOST").unwrap_or("http://localhost:".to_string());
        let port = env::var("port").unwrap_or("8080".to_string());
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

