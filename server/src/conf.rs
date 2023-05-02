use std::{env, fs::File, fmt::{Display, Formatter, self}, path::Path};

use simplelog::*;
use dotenv::dotenv;
use hmac::{Hmac, Mac};
use serde::Serialize;
use sha2::Sha256;

const VARLOG_DIR: &str = "/var/log";

#[derive(Debug)]
pub struct ConfigError(String);

impl std::error::Error for ConfigError {}

impl Display for ConfigError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Clone)]
pub struct AppConfig {
    pub key: Hmac<Sha256>,
    pub registry_url: String,
    pub log_dir: String,
}

impl AppConfig {
    pub async fn configure() -> Result<Self, ConfigError>  {
        Self::init_loggers();
        dotenv().ok();

        let key = Hmac::new_from_slice(
            Self::get_env_var("JWT_SIGNING_KEY")?
            .as_bytes(),
        ).map_err(|e| ConfigError(
            format!("JWT key could not be hashed: {}.", e),
        ))?;

        let registry_url = Self::get_env_var("REGISTRY_URL")?;
        let hostname = Self::get_env_var("HOSTNAME")?;


        Self::attempt_register_hostname(
            hostname,
            registry_url.clone(),
        ).await;

        let log_dir = VARLOG_DIR.to_string();
        Ok(Self {
            key,
            registry_url,
            log_dir,
        })
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
        let varlog_logfile = Path::new(VARLOG_DIR).join("varlog.log");
        let file_err = match File::create(&varlog_logfile)  {
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
                    "Could not create log file {:?}: {}.", 
                    varlog_logfile,
                    e,
                ))
            }
        };

        if let Err(e) = CombinedLogger::init(loggers) {
            warn!("Could not init loggers: {}.", e);
        }

        if let Some(e) = file_err {
            // Need to delay logging until loggers have been initialized
            warn!("{}", e);
        }
    }

    async fn attempt_register_hostname(
            hostname: String,
            registry_url: String,
        ) {
        #[derive(Serialize)]
        struct RegistryRequest {
            hostname: String,
        }

        let resp = reqwest::Client::new()
            .post(format!("{registry_url}/register"))
            .json(&RegistryRequest {
                hostname,
            })
            .send()
            .await;

        if let Err(e) = resp {
            warn!("Error while registering hostname: {}.", e);
        } else {
            info!("Successfully registered hostname.");
        }
    }

    fn get_env_var(v: &str) -> Result<String, ConfigError> {
        env::var(v).map_err(|_| ConfigError(
            format!("{v} should be found in the environment.")
        ))
    }
}
