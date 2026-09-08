use anyhow::{Result, anyhow};
use serde_json::{Value, json};

use super::registry::Tool;

pub struct Calculator;

#[async_trait::async_trait]
impl Tool for Calculator {
    fn name(&self) -> &str {
        "calculator"
    }

    fn description(&self) -> &str {
        "Evaluate a mathematical expression."
    }

    async fn execute(&self, input: Value) -> Result<Value> {
        let expression = input
            .get("expression")
            .and_then(Value::as_str)
            .ok_or_else(|| anyhow!("Missing 'expression'"))?;

        // Temporary implementation.
        // We will replace this with a real expression parser.
        let result = match expression {
            "2 + 2" => 4.0,
            "10 * 5" => 50.0,
            "100 / 4" => 25.0,
            _ => return Err(anyhow!("Unsupported expression: {expression}")),
        };

        Ok(json!({
            "result": result
        }))
    }
    fn parameters(&self) -> Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate"
                }
            },
            "required": ["expression"]
        })
    }
}
