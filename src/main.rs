use rand::Rng;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Deserialize, Serialize)]
struct Monitor {
    name: String,
    #[serde(default)]
    script: Option<String>,
    #[serde(default)]
    result: Option<Result>,
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

#[derive(Debug, Deserialize, Serialize)]
struct Result {
    value: i32,
    processed_at: i64,
}

fn main() {
    let wrong_command =
        "💥💥 Wrong Command! Usage: process_monitor -monitorFile /path/to/given/monitors.json";
    // Get the command-line arguments
    let args: Vec<String> = env::args().collect();

    println!("ARGS ->> {:?}", args);

    // Check if the argument count is correct
    if args.len() != 4 {
        println!("{}", wrong_command);
        return;
    }

    // if second arg is not process_monitor show error
    if args[1] != "process_monitor" {
        println!("{}", wrong_command);
        return;
    }
    // if third arg is not -monitorFile show error
    if args[2] != "-monitorFile" {
        println!("{}", wrong_command);
        return;
    }

    // Get the file path from the second argument
    let file_path = &args[3];

    let content = match std::fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(err) => {
            println!("❗❗ Could not read file `{}`: {}", file_path, err);
            return;
        }
    };

    // Deserialize the JSON data into a Monitors struct
    let mut monitors: Monitors = match serde_json::from_str(&content) {
        Ok(monitors) => monitors,
        Err(err) => {
            println!(
                "❗❗ Failed to parse JSON data from file `{}`: {}",
                file_path, err
            );
            return;
        }
    };

    // adding the result for each struct in the monitors
    for monitor in &mut monitors.monitors {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Wrong time");
        // creating the result struct
        let result: Result = Result {
            value: rand::thread_rng().gen_range(5..100), // generates random value in a range of 5 to 100
            processed_at: since_the_epoch.as_secs() as i64, // generates the time in seconds
        };
        monitor.result = Some(result); // adding the result in each monitor
    }

    // serialize the monitors to JSON
    let new_json =
        serde_json::to_string_pretty(&monitors).expect("Failed to serialize monitors to JSON");

    // write the JSON string to a file
    let mut file = BufWriter::new(
        File::create("assets/updated_monitors.json").expect("Unable to create file"),
    );
    // write the json to the assets folder
    file.write_all(new_json.as_bytes())
        .expect("Failed to write JSON to file");
}
