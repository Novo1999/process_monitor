use anyhow::Context;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize)]
struct Monitor {
    name: String,
    #[serde(default)]
    script: Option<String>,
    #[serde(default)]
    result: Option<String>,
    code: String,
    #[serde(default)]
    monitor_id: Option<u32>,
    #[serde(default)]
    #[serde(rename = "type")]
    monitor_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
struct Monitors {
    monitors: Vec<Monitor>,
}

// struct Result {
//     value: i32,
//     processed_at: i64,
// }

fn main() {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    println!("ARGS ->> {:?}", args);

    // Check if the argument count is correct
    if args.len() != 4 {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
        return;
    }

    if args[1] != "process_monitor" {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
        return;
    }

    if args[2] != "-monitorFile" {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
        return;
    }

    // Get the file path from the second argument
    let file_path = &args[3];

    let content: String = std::fs::read_to_string(file_path)
        .with_context(|| format!("Could not read file `{}`", file_path))?; // this shows a nice error by using the anyhow library

    let monitors: Monitors = serde_json::from_str(&content)
        .with_context(|| format!("Failed to parse JSON data from file `{}`", file_path))?;

    println!("{:?}", monitors);
}
