use anyhow::{Context, Result};
use serde_json::{Value, json};
use tokio::fs;

use super::registry::Tool;

pub struct FileWrite;

#[async_trait::async_trait]
impl Tool for FileWrite {
    fn name(&self) -> &str {
        "file_write"
    }

    fn description(&self) -> &str {
        "Create a file or overwrite an existing file with the provided content."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path of the file to create or write"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write into the file"
                }
            },
            "required": ["path", "content"]
        })
    }

    async fn execute(&self, input: Value) -> Result<Value> {
        let path = input
            .get("path")
            .and_then(Value::as_str)
            .context("Missing 'path'")?;

        let content = input
            .get("content")
            .and_then(Value::as_str)
            .context("Missing 'content'")?;

        fs::write(path, content)
            .await
            .with_context(|| format!("Failed to write file: {path}"))?;

        Ok(json!({
            "success": true,
            "path": path,
            "message": "File written successfully"
        }))
    }
}
