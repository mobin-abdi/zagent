use anyhow::{Context, Result};
use serde_json::{Value, json};
use tokio::fs;

use super::registry::Tool;

pub struct FileRead;

#[async_trait::async_trait]
impl Tool for FileRead {
    fn name(&self) -> &str {
        "file_read"
    }

    fn description(&self) -> &str {
        "Read the contents of a file."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path of the file to read"
                }
            },
            "required": ["path"]
        })
    }

    async fn execute(&self, input: Value) -> Result<Value> {
        let path = input
            .get("path")
            .and_then(Value::as_str)
            .context("Missing 'path'")?;

        let content = fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read file: {path}"))?;

        Ok(json!({
            "path": path,
            "content": content
        }))
    }
}
