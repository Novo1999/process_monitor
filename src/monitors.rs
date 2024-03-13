use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Deserialize, Serialize)]
pub struct Monitors {
    pub monitors: Vec<Monitor>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Monitor {
    pub name: String,
    #[serde(default)]
    pub script: Option<String>,
    #[serde(default)]
    pub result: Option<ResultVal>,
    pub code: String,
    #[serde(default)]
    pub monitor_id: Option<u32>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub monitor_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ResultVal {
    pub value: i32,
    pub processed_at: i64,
}

pub fn get_monitors(file_path: &str) -> Result<Monitors> {
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("❗❗ Could not read file `{}`", file_path))?;

    let monitors: Monitors = serde_json::from_str(&content)
        .with_context(|| format!("❗❗ Failed to parse JSON data from file `{}`", file_path))?;

    Ok(monitors)
}
