use std::env;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

fn main() -> Result<()> {
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    println!("ARGS ->> {:?}", args);

    // Check if the argument count is correct
    if args.len() != 4 {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
    }

    // Check if the first argument is "-monitorFile"
    if args[1] != "process_monitor" {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
    }

    if args[2] != "-monitorFile" {
        println!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
    }

    // Get the file path from the second argument
    let file_path = &args[3];

    let content: String = std::fs::read_to_string(file_path)
        .with_context(|| format!("Could not read file `{}`", file_path))?; // this shows a nice error by using the anyhow library

    // Use the file path as needed
    println!("File path: {}", file_path);
    println!("{}", content);

    Ok(())
}
