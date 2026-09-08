mod app;
mod ui;

use crate::agent::{self, agent::Agent};
use anyhow::Result;

pub async fn run(agent: Agent) -> Result<()> {
    app::run(agent).await
}
