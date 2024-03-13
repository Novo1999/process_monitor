use std::time::{SystemTime, UNIX_EPOCH};

use anyhow::Result;
use rand::Rng;

use crate::write_to_file::write_to_file;
mod monitors;
mod write_to_file;

fn main() -> Result<()> {
    let wrong_command =
        "💥💥 Wrong Command! Usage: process_monitor -monitorFile /path/to/given/monitors.json";
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    println!("ARGS ->> {:?}", args);

    // Check if the argument count is correct
    if args.len() != 4 {
        println!("{}", wrong_command);
        return Ok(());
    }

    // if second arg is not process_monitor show error
    if args[1] != "process_monitor" {
        println!("{}", wrong_command);
        return Ok(());
    }
    // if third arg is not -monitorFile show error
    if args[2] != "-monitorFile" {
        println!("{}", wrong_command);
        return Ok(());
    }

    // Get the file path from the second argument
    let file_path = &args[3];

    // Get monitors from the file
    let mut monitors = monitors::get_monitors(file_path)?;

    // Add results to monitors
    for monitor in &mut monitors.monitors {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).expect("Wrong time");
        // creating the result struct
        let result = monitors::ResultVal {
            value: rand::thread_rng().gen_range(5..100), // generates random value in a range of 5 to 100
            processed_at: since_the_epoch.as_secs() as i64, // generates the time in seconds
        };
        monitor.result = Some(result); // adding the result in each monitor
    }

    // Write monitors to JSON file
    if let Err(err) = write_to_file(&monitors, "assets/updated_monitors.json") {
        println!("❗❗ Error writing to file: {}", err);
    }

    Ok(())
}
