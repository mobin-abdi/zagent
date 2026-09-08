use anyhow::{Result, anyhow};
use serde_json::Value;

use super::{message::Message, state::AgentState, tool_call::ToolCall};

use crate::{llm::client::LlmClient, tools::registry::ToolRegistry};

pub struct Agent {
    pub state: AgentState,
    pub tools: ToolRegistry,
    pub llm: LlmClient,
}

impl Agent {
    pub fn new(tools: ToolRegistry, llm: LlmClient) -> Self {
        Self {
            state: AgentState::new(),
            tools,
            llm,
        }
    }

    pub fn add_message(&mut self, message: Message) {
        self.state.add_message(message);
    }

    pub async fn run(&mut self) -> Result<String> {
        const MAX_ITERATIONS: usize = 10;

        for _ in 0..MAX_ITERATIONS {
            self.state.next_iteration();

            let response = self.llm.chat(&self.state.messages, &self.tools).await?;

            let message = response
                .choices
                .first()
                .ok_or_else(|| anyhow!("LLM returned no choices"))?
                .message
                .clone();

            if let Some(tool_calls) = message.tool_calls.filter(|calls| !calls.is_empty()) {
                let calls: Vec<ToolCall> = tool_calls
                    .into_iter()
                    .map(|call| {
                        let arguments: Value = serde_json::from_str(&call.function.arguments)?;

                        Ok(ToolCall {
                            id: call.id,
                            name: call.function.name,
                            arguments,
                        })
                    })
                    .collect::<Result<Vec<_>>>()?;

                self.state
                    .add_message(Message::assistant_with_tools(calls.clone()));

                for call in calls {
                    self.state.record_tool_call();

                    let result = self.tools.execute(&call.name, call.arguments).await?;

                    self.state
                        .add_message(Message::tool(call.id, call.name, result.to_string()));
                }

                continue;
            }

            let content = message.content.unwrap_or_default();

            self.state.add_message(Message::assistant(content.clone()));

            return Ok(content);
        }

        Err(anyhow!("Agent exceeded maximum iterations"))
    }
}
