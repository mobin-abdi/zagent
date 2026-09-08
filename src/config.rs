use std::{env, path::PathBuf};

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "zagent",
    version,
    about = "A resource-efficient AI coding agent"
)]
pub struct Cli {
    /// Path to the environment file
    #[arg(long, value_name = "PATH")]
    pub env: Option<PathBuf>,
}

pub struct Config {
    pub api_key: String,
    pub api_url: String,
    pub model: String,
}

impl Config {
    pub fn from_cli(cli: &Cli) -> Result<Self> {
        if let Some(path) = &cli.env {
            dotenvy::from_path(path)
                .with_context(|| format!("Failed to load environment file: {}", path.display()))?;
        } else {
            dotenvy::dotenv().ok();
        }

        let api_key = env::var("ZAGENT_API_KEY").context("ZAGENT_API_KEY is not set")?;

        let api_url = env::var("ZAGENT_BASE_URL").context("ZAGENT_BASE_URL is not set")?;

        let model = env::var("ZAGENT_MODEL").context("ZAGENT_MODEL is not set")?;

        Ok(Self {
            api_key,
            api_url,
            model,
        })
    }
}
