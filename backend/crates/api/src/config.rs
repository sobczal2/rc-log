use std::env;
use std::net::SocketAddr;
use std::path::PathBuf;

struct ConfigNames {
    pub app_env: &'static str,
    pub database_url: &'static str,
    pub host: &'static str,
    pub port: &'static str,
    pub asset_path: &'static str,
    pub jwt_secret: &'static str,
    pub asset_cache_size: &'static str,
}

const CONFIG_NAMES: ConfigNames = ConfigNames {
    app_env: "RC_LOG_ENV",
    database_url: "RC_LOG_DATABASE_URL",
    host: "RC_LOG_HOST",
    port: "RC_LOG_PORT",
    asset_path: "RC_LOG_ASSET_PATH",
    jwt_secret: "RC_LOG_JWT_SECRET",
    asset_cache_size: "RC_LOG_ASSET_CACHE_SIZE",
};

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
    pub asset_path: PathBuf,
    pub jwt_secret: String,
    pub asset_cache_size: u64,
}

impl AppConfig {
    pub fn load() -> Self {
        Self::from_env_vars()
    }

    fn from_env_vars() -> Self {
        let environment =
            Environment::from_str(&env::var(CONFIG_NAMES.app_env).expect("RC_LOG_ENV must be set"));

        let database_url =
            env::var(CONFIG_NAMES.database_url).expect("RC_LOG_DATABASE_URL must be set");

        let host = env::var(CONFIG_NAMES.host).expect("RC_LOG_HOST must be set");

        let port = env::var(CONFIG_NAMES.port)
            .expect("RC_LOG_PORT must be set")
            .parse::<u16>()
            .expect("RC_LOG_PORT must be a valid u16");

        let asset_path =
            env::var(CONFIG_NAMES.asset_path).expect("RC_LOG_ASSET_PATH must be set").into();

        let jwt_secret = env::var(CONFIG_NAMES.jwt_secret).expect("RC_LOG_JWT_SECRET must be set");

        let asset_cache_size = env::var(CONFIG_NAMES.asset_cache_size)
            .expect("RC_LOG_ASSET_CACHE_SIZE must be set")
            .parse::<u64>()
            .expect("RC_LOG_ASSET_CACHE_SIZE must be a valid u64");

        Self { environment, database_url, host, port, asset_path, jwt_secret, asset_cache_size }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port).parse().expect("Invalid host/port combination")
    }
}
