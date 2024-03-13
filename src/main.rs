use crate::write_to_file::write_to_file;
use anyhow::Result as AnyhowResult;
use rand::Rng;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// module declarations
mod monitors;
mod write_to_file;

fn main() -> AnyhowResult<()> {
    let wrong_command =
        "💥💥 Wrong Command! Usage: process_monitor -monitorFile /path/to/given/monitors.json";

    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    let sliced_args: &[String] = &args[1..args.len()]; // ignoring the first value as it is redundant here

    // Check if the argument count is correct
    if sliced_args.len() != 3 {
        println!("{}", wrong_command);
        return Ok(());
    }

    // if first arg is not process_monitor show error
    if sliced_args[0] != "process_monitor" {
        println!("{}", "💥💥 Incorrect Executable File Name!");
        return Ok(());
    }
    // if second arg is not -monitorFile show error
    if sliced_args[1] != "-monitorFile" {
        println!("{}", wrong_command);
        return Ok(());
    }

    // Get the file path from the third argument
    let file_path: &String = &sliced_args[2];

    // Get monitors from the file
    let mut monitors: monitors::Monitors = monitors::get_monitors(file_path)?;

    // Add results to monitors
    for monitor in &mut monitors.monitors {
        let start: SystemTime = SystemTime::now();
        let since_the_epoch: Duration = start.duration_since(UNIX_EPOCH).expect("Wrong time");
        // creating the result struct
        let result: monitors::Result = monitors::Result {
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
