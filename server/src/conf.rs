use std::{env, fs::File};

use simplelog::*;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;

const VARLOG_LOG_FILE: &str = "/var/log/varlog.log";

#[derive(Serialize)]
struct RegistryRequest {
    hostname: String,
}

#[derive(Clone)]
pub struct AppConfig {
    pub key: Hmac<Sha256>,
    pub registry_url: String,
    hostname: String,
}

impl AppConfig {
    pub fn new()  -> Result<Self, ConfigError>  {
        dotenv().ok();

        let key = Hmac::new_from_slice(
            get_env_var("JWT_SIGNING_KEY")?
            .as_bytes(),
        ).map_err(|e| ConfigError(
            format!("JWT key could not be hashed: {}.", e),
        ))?;

        let registry_url = get_env_var("REGISTRY_URL")?;
        let hostname = get_env_var("HOSTNAME")?;

        Ok(Self {
            key,
            registry_url,
            hostname,
        })
    }

    pub async fn configure(&self) {
        Self::init_loggers();
        self.register_hostname().await;
    }

    fn init_loggers() {
        let mut loggers:  Vec<Box<dyn SharedLogger>> = vec![
            TermLogger::new(
                LevelFilter::Info,
                Config::default(),
                TerminalMode::Mixed,
                ColorChoice::Auto,
            ),
        ];

        let file_err = match File::create(VARLOG_LOG_FILE)  {
            Ok(file) => {
                loggers.push(
                    WriteLogger::new(
                        LevelFilter::Info,
                        Config::default(),
                        file,
                    )
                );
                None
            },
            Err(e) => {
                Some(format!(
                    "Could not create log file {}: {}.", 
                    VARLOG_LOG_FILE,
                    e,
                ))
            }
        };

        if let Err(e) = CombinedLogger::init(loggers) {
            error!("Could not init loggers: {}.", e);
        }

        if let Some(e) = file_err {
            // Need to delay logging until loggers have been initialized
            warn!("{}", e);
        }
    }

    async fn register_hostname(&self) {
        info!("Registering hostname...");

        let hostname = self.hostname.clone();
        let registry_url = self.registry_url.clone();

        let resp = reqwest::Client::new()
            .post(format!("{registry_url}/register"))
            .json(&RegistryRequest {
                hostname,
            })
            .send()
            .await;

        if let Err(e) = resp {
            error!("Error while registering hostname: {}.", e);
        } else {
            info!("Successfully registered hostname.");
        }
    }
}

fn get_env_var(v: &str) -> Result<String, ConfigError> {
    env::var(v).map_err(|_|  ConfigError(format!(
        "{v} should be found in the environment."
    )))
}

#[derive(Debug)]
pub struct ConfigError(String);

impl std::fmt::Display  for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self)
    }
}

impl std::error::Error for ConfigError {
    fn description(&self) -> &str {
        &self.0.as_str()
    }
}
