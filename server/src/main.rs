#[macro_use] extern crate log;

use anyhow::Result;

use server::{run, conf::AppConfig};

#[actix_web::main]
async fn main() -> Result<()> {
    let config = match AppConfig::configure().await {
        Ok(config) => config,
        Err(e) => {
            error!("Configuration Error: {e}");
            std::process::exit(1);
        }
    };
    run(config).await?;
    Ok(())
}
