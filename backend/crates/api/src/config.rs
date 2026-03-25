use std::net::SocketAddr;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    Development,
    Production,
}

impl Environment {
    fn from_str(s: &str) -> Self {
        match s {
            "production" | "prod" => Environment::Production,
            "development" | "dev" => Environment::Development,
            other => panic!(
                "Unknown APP_ENV value: '{}'. Expected 'development', 'dev', 'production', or 'prod'",
                other
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub environment: Environment,
    pub database_url: String,
    pub host: String,
    pub port: u16,
}

impl AppConfig {
    pub fn load() -> Self {
        Self::from_env_vars()
    }

    fn from_env_vars() -> Self {
        let environment =
            Environment::from_str(&std::env::var("APP_ENV").expect("APP_ENV must be set"));

        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let host = std::env::var("APP_HOST").expect("APP_HOST must be set");

        let port = std::env::var("APP_PORT")
            .expect("APP_PORT must be set")
            .parse::<u16>()
            .expect("APP_PORT must be a valid u16");

        Self { environment, database_url, host, port }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port).parse().expect("Invalid host/port combination")
    }
}
