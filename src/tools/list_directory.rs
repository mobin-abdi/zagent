use anyhow::{Context, Result};
use serde_json::{Value, json};
use tokio::fs;

use super::registry::Tool;

pub struct ListDirectory;

#[async_trait::async_trait]
impl Tool for ListDirectory {
    fn name(&self) -> &str {
        "list_directory"
    }

    fn description(&self) -> &str {
        "List the files and directories inside a directory."
    }

    fn parameters(&self) -> Value {
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "Path of the directory to list"
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

        let mut entries = fs::read_dir(path)
            .await
            .with_context(|| format!("Failed to read directory: {path}"))?;

        let mut items = Vec::new();

        while let Some(entry) = entries
            .next_entry()
            .await
            .context("Failed to read directory entry")?
        {
            let file_type = entry.file_type().await.context("Failed to get file type")?;

            let name = entry.file_name().to_string_lossy().to_string();

            items.push(json!({
                "name": name,
                "type": if file_type.is_dir() {
                    "directory"
                } else if file_type.is_file() {
                    "file"
                } else {
                    "other"
                }
            }));
        }

        Ok(json!({
            "path": path,
            "entries": items
        }))
    }
}
