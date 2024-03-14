use crate::{error_handler::USAGE_COMMAND, write_to_file::write_to_file};
use anyhow::{Context, Result as AnyhowResult};
use error_handler::check_error_cases;
use rand::Rng;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

// module declarations
mod error_handler;
mod monitors;
mod write_to_file;

fn main() -> AnyhowResult<()> {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    println!("{:?}", args);

    // Check if enough arguments are provided
    if args.len() == 1 {
        println!("💥 NO ARGUMENTS PROVIDED! / {}", USAGE_COMMAND);
        return Ok(());
    }

    if args.len() == 2 {
        println!("💥💥 Very few arguments! / {}", USAGE_COMMAND);
        return Ok(());
    }

    // Get the file path from the third argument
    let file_path: &String = &args[2];

    // Check if the file path is provided
    if file_path.is_empty() {
        println!("File path is empty.");
        return Ok(());
    }

    // this will handle more errors based on multiple wrong user input cases
    check_error_cases(&args[1..]).ok();

    // Get monitors from the file
    let mut monitors: monitors::Monitors = monitors::get_monitors(file_path)
        .with_context(|| format!("Failed to get monitors from file: {}", file_path))?;

    // Add results to monitors
    for monitor in &mut monitors.monitors {
        let start: SystemTime = SystemTime::now();
        let since_the_epoch: Duration = start
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!"); // This should never happen
                                                      // creating the result struct
        let result: monitors::Result = monitors::Result {
            value: rand::thread_rng().gen_range(5..100), // generating random value in a range of 5 to 100
            processed_at: since_the_epoch.as_secs() as i64, // generating the time in seconds
        };
        monitor.result = Some(result); // adding the result in each monitor
    }

    // Write monitors to JSON file
    write_to_file(&monitors, "assets/updated_monitors.json")
        .with_context(|| "Failed to write monitors to file")?;

    Ok(())
}
