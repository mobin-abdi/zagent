use anyhow::{Context, Result};
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    agent::message::{Message, Role},
    tools::registry::ToolRegistry,
};

use serde_json::Value;

#[derive(Clone)]
pub struct LlmClient {
    client: Client,
    api_url: String,
    api_key: String,
    model: String,
}

impl LlmClient {
    pub fn new(
        api_url: impl Into<String>,
        api_key: impl Into<String>,
        model: impl Into<String>,
    ) -> Self {
        Self {
            client: Client::new(),
            api_url: api_url.into(),
            api_key: api_key.into(),
            model: model.into(),
        }
    }

    pub async fn chat(&self, messages: &[Message], tools: &ToolRegistry) -> Result<LlmResponse> {
        let request = ChatRequest {
            model: self.model.clone(),
            messages: messages.iter().map(ApiMessage::from).collect(),
            tools: tools.definitions(),
        };

        let response = self
            .client
            .post(&self.api_url)
            .bearer_auth(&self.api_key)
            .json(&request)
            .send()
            .await
            .context("Failed to send request to LLM")?;

        let status = response.status();

        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();

            anyhow::bail!("LLM request failed: {} - {}", status, body);
        }

        let body = response
            .text()
            .await
            .context("Failed to read LLM response body")?;

        let response: LlmResponse =
            serde_json::from_str(&body).context("Failed to parse LLM response")?;

        Ok(response)
    }
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<ApiMessage>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    tools: Vec<Value>,
}

#[derive(Debug, Serialize)]
struct ApiMessage {
    role: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    content: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ApiToolCall>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    tool_call_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
}

impl From<&Message> for ApiMessage {
    fn from(message: &Message) -> Self {
        Self {
            role: match message.role {
                Role::System => "system",
                Role::User => "user",
                Role::Assistant => "assistant",
                Role::Tool => "tool",
            }
            .to_string(),

            content: message.content.clone(),

            tool_calls: message.tool_calls.as_ref().map(|calls| {
                calls
                    .iter()
                    .map(|call| ApiToolCall {
                        id: call.id.clone(),
                        tool_type: "function".to_string(),
                        function: ApiToolCallFunction {
                            name: call.name.clone(),
                            arguments: call.arguments.to_string(),
                        },
                    })
                    .collect()
            }),

            tool_call_id: message.tool_call_id.clone(),
            name: message.name.clone(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct LlmResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseMessage {
    pub role: String,
    pub content: Option<String>,
    pub tool_calls: Option<Vec<ResponseToolCall>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseToolCall {
    pub id: String,
    pub function: ResponseFunction,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ResponseFunction {
    pub name: String,
    pub arguments: String,
}

#[derive(Debug, Serialize)]
struct ApiToolCall {
    id: String,
    #[serde(rename = "type")]
    tool_type: String,
    function: ApiToolCallFunction,
}

#[derive(Debug, Serialize)]
struct ApiToolCallFunction {
    name: String,
    arguments: String,
}
