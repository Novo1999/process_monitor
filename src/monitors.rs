use crate::file_writer::write_to_file;
use crate::monitors;
use anyhow::{Context, Result as AnyhowResult};
use chrono::Local;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::time::interval;

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
    pub result: Option<Result>,
    pub code: String,
    #[serde(default)]
    pub monitor_id: Option<u32>,
    #[serde(default)]
    #[serde(rename = "type")]
    pub monitor_type: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Result {
    pub value: i32,
    pub processed_at: i64,
}

// get the monitors json file
pub fn get_monitors(file_path: &str) -> AnyhowResult<Monitors> {
    // read as string
    let content = fs::read_to_string(file_path)
        .with_context(|| format!("❗❗ Could not read file `{}`", file_path))?;

    // get the monitors data
    let monitors: Monitors = serde_json::from_str(&content)
        .with_context(|| format!("❗❗ Failed to parse JSON data from file `{}`", file_path))?;

    Ok(monitors)
}

// this updates the monitors
pub async fn update_monitors(monitors: Arc<Mutex<Monitors>>) {
    let mut interval = interval(Duration::from_secs(30));

    loop {
        interval.tick().await;
        println!("Running update monitors");

        // Update monitors data
        let mut monitors = monitors.lock().unwrap();
        for monitor in &mut monitors.monitors {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX EPOCH!");
            let result = monitors::Result {
                value: rand::thread_rng().gen_range(5..100), // generating random value in a range of 5 to 100
                processed_at: since_the_epoch.as_secs() as i64, // generating the time in seconds
            };
            monitor.result = Some(result); // adding the result in each monitor
        }

        // Write monitors to JSON file
        if let Err(err) = write_to_file(&*monitors, "assets/monitors_with_result.json") {
            println!("Failed to write monitors to file: {}", err);
        } else {
            println!("Monitors successfully updated.");
        }
    }
}

// this stores the monitors
pub async fn store_monitors(monitors: Arc<Mutex<Monitors>>) {
    let mut interval = interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        println!("Running store monitors");

        // Update monitors data
        let mut monitors = monitors.lock().unwrap();
        for monitor in &mut monitors.monitors {
            let start = SystemTime::now();
            let since_the_epoch = start
                .duration_since(UNIX_EPOCH)
                .expect("SystemTime before UNIX EPOCH!");
            let result = monitors::Result {
                value: rand::thread_rng().gen_range(5..100), // generating random value in a range of 5 to 100
                processed_at: since_the_epoch.as_secs() as i64, // generating the time in seconds
            };
            monitor.result = Some(result); // adding the result in each monitor
        }

        // Format the current time in 12-hour format with AM/PM indicator
        let current_time_formatted = Local::now().format("%I_%M%p").to_string();

        // Write monitors to JSON file with the formatted current time
        if let Err(err) = write_to_file(
            &monitors,
            &format!("assets/{}_monitors.json", current_time_formatted),
        ) {
            println!("Failed to write monitors to file: {}", err);
        } else {
            println!("Monitors successfully stored.");
        }
    }
}
