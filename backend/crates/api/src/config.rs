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
    pub model_cache_ttl_seconds: &'static str,
    pub model_cache_size: &'static str,
    pub maneuver_cache_ttl_seconds: &'static str,
    pub maneuver_cache_size: &'static str,
    pub variation_cache_ttl_seconds: &'static str,
    pub variation_cache_size: &'static str,
    pub video_cache_ttl_seconds: &'static str,
    pub video_cache_size: &'static str,
    pub photo_cache_ttl_seconds: &'static str,
    pub photo_cache_size: &'static str,
}

const CONFIG_NAMES: ConfigNames = ConfigNames {
    app_env: "RC_LOG_ENV",
    database_url: "RC_LOG_DATABASE_URL",
    host: "RC_LOG_HOST",
    port: "RC_LOG_PORT",
    asset_path: "RC_LOG_ASSET_PATH",
    jwt_secret: "RC_LOG_JWT_SECRET",
    model_cache_ttl_seconds: "RC_LOG_MODEL_CACHE_TTL_SECONDS",
    model_cache_size: "RC_LOG_MODEL_CACHE_SIZE",
    maneuver_cache_ttl_seconds: "RC_LOG_MANEUVER_CACHE_TTL_SECONDS",
    maneuver_cache_size: "RC_LOG_MANEUVER_CACHE_SIZE",
    variation_cache_ttl_seconds: "RC_LOG_VARIATION_CACHE_TTL_SECONDS",
    variation_cache_size: "RC_LOG_VARIATION_CACHE_SIZE",
    video_cache_ttl_seconds: "RC_LOG_VIDEO_CACHE_TTL_SECONDS",
    video_cache_size: "RC_LOG_VIDEO_CACHE_SIZE",
    photo_cache_ttl_seconds: "RC_LOG_PHOTO_CACHE_TTL_SECONDS",
    photo_cache_size: "RC_LOG_PHOTO_CACHE_SIZE",
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
    pub model_cache_ttl_seconds: u64,
    pub model_cache_size: u64,
    pub maneuver_cache_ttl_seconds: u64,
    pub maneuver_cache_size: u64,
    pub variation_cache_ttl_seconds: u64,
    pub variation_cache_size: u64,
    pub video_cache_ttl_seconds: u64,
    pub video_cache_size: u64,
    pub photo_cache_ttl_seconds: u64,
    pub photo_cache_size: u64,
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

        let model_cache_ttl_seconds = env::var(CONFIG_NAMES.model_cache_ttl_seconds)
            .expect("RC_LOG_MODEL_CACHE_TTL_SECONDS must be set")
            .parse::<u64>()
            .expect("RC_LOG_MODEL_CACHE_TTL_SECONDS must be a valid u64");

        let model_cache_size = env::var(CONFIG_NAMES.model_cache_size)
            .expect("RC_LOG_MODEL_CACHE_SIZE must be set")
            .parse::<u64>()
            .expect("RC_LOG_MODEL_CACHE_SIZE must be a valid u64");

        let maneuver_cache_ttl_seconds = env::var(CONFIG_NAMES.maneuver_cache_ttl_seconds)
            .expect("RC_LOG_MANEUVER_CACHE_TTL_SECONDS must be set")
            .parse::<u64>()
            .expect("RC_LOG_MANEUVER_CACHE_TTL_SECONDS must be a valid u64");

        let maneuver_cache_size = env::var(CONFIG_NAMES.maneuver_cache_size)
            .expect("RC_LOG_MANEUVER_CACHE_SIZE must be set")
            .parse::<u64>()
            .expect("RC_LOG_MANEUVER_CACHE_SIZE must be a valid u64");

        let variation_cache_ttl_seconds = env::var(CONFIG_NAMES.variation_cache_ttl_seconds)
            .expect("RC_LOG_VARIATION_CACHE_TTL_SECONDS must be set")
            .parse::<u64>()
            .expect("RC_LOG_VARIATION_CACHE_TTL_SECONDS must be a valid u64");

        let variation_cache_size = env::var(CONFIG_NAMES.variation_cache_size)
            .expect("RC_LOG_VARIATION_CACHE_SIZE must be set")
            .parse::<u64>()
            .expect("RC_LOG_VARIATION_CACHE_SIZE must be a valid u64");

        let video_cache_ttl_seconds = env::var(CONFIG_NAMES.video_cache_ttl_seconds)
            .expect("RC_LOG_VIDEO_CACHE_TTL_SECONDS must be set")
            .parse::<u64>()
            .expect("RC_LOG_VIDEO_CACHE_TTL_SECONDS must be a valid u64");

        let video_cache_size = env::var(CONFIG_NAMES.video_cache_size)
            .expect("RC_LOG_VIDEO_CACHE_SIZE must be set")
            .parse::<u64>()
            .expect("RC_LOG_VIDEO_CACHE_SIZE must be a valid u64");

        let photo_cache_ttl_seconds = env::var(CONFIG_NAMES.photo_cache_ttl_seconds)
            .expect("RC_LOG_PHOTO_CACHE_TTL_SECONDS must be set")
            .parse::<u64>()
            .expect("RC_LOG_PHOTO_CACHE_TTL_SECONDS must be a valid u64");

        let photo_cache_size = env::var(CONFIG_NAMES.photo_cache_size)
            .expect("RC_LOG_PHOTO_CACHE_SIZE must be set")
            .parse::<u64>()
            .expect("RC_LOG_PHOTO_CACHE_SIZE must be a valid u64");

        Self {
            environment,
            database_url,
            host,
            port,
            asset_path,
            jwt_secret,
            model_cache_ttl_seconds,
            model_cache_size,
            maneuver_cache_ttl_seconds,
            maneuver_cache_size,
            variation_cache_ttl_seconds,
            variation_cache_size,
            video_cache_ttl_seconds,
            video_cache_size,
            photo_cache_ttl_seconds,
            photo_cache_size,
        }
    }

    pub fn socket_addr(&self) -> SocketAddr {
        format!("{}:{}", self.host, self.port).parse().expect("Invalid host/port combination")
    }
}
