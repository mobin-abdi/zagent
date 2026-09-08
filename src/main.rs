mod agent;
mod config;
mod llm;
mod tools;
mod tui;

use clap::Parser;

use agent::{agent::Agent, message::Message};

use config::{Cli, Config};

use llm::client::LlmClient;

use tools::{
    calculator::Calculator, file_read::FileRead, file_write::FileWrite,
    list_directory::ListDirectory, registry::ToolRegistry,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let config = Config::from_cli(&cli)?;

    let llm = LlmClient::new(config.api_url, config.api_key, config.model);

    let mut tools = ToolRegistry::new();

    tools.register(Calculator);
    tools.register(FileRead);
    tools.register(ListDirectory);
    tools.register(FileWrite);

    let mut agent = Agent::new(tools, llm);

    agent.add_message(Message::system(
        "You are ZAgent, a helpful AI coding agent.",
    ));

    tui::run(agent).await?;

    Ok(())
}
