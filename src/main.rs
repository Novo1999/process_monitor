use crate::{error_handler::USAGE_COMMAND, file_writer::write_to_file};
use anyhow::{Context, Result as AnyhowResult};
use error_handler::check_error_cases;
use monitors::Monitors;
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::{spawn, time::interval};

use chrono::Local;

// module declarations
mod error_handler;
mod file_writer;
mod monitors;
mod update_monitors;

#[tokio::main]
async fn main() -> AnyhowResult<()> {
    // Get the command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if enough arguments are provided
    if args.len() == 1 {
        println!("💥 NO ARGUMENTS PROVIDED! / {}", USAGE_COMMAND);
        return Ok(());
    }

    if args.len() == 2 || args.len() == 3 {
        println!("💥💥 Very few arguments! / {}", USAGE_COMMAND);
        return Ok(());
    }

    // Get the file path from the third argument
    let file_path: &String = &args[3];

    // Check if the file path is provided
    if file_path.is_empty() {
        println!("📁 File path is empty.");
        return Ok(());
    }

    // Get monitors from the file
    let monitors: Arc<Mutex<Monitors>> = Arc::new(Mutex::new(
        monitors::get_monitors(file_path)
            .with_context(|| format!("Failed to get monitors from file: {}", file_path))?,
    ));

    // Add results to monitors
    for monitor in &mut monitors.lock().unwrap().monitors {
        let start: SystemTime = SystemTime::now();
        let since_the_epoch: Duration = start
            .duration_since(UNIX_EPOCH)
            .expect("SystemTime before UNIX EPOCH!");
        // creating the result struct
        let result: monitors::Result = monitors::Result {
            value: rand::thread_rng().gen_range(5..100), // generating random value in a range of 5 to 100
            processed_at: since_the_epoch.as_secs() as i64, // generating the time in seconds
        };
        monitor.result = Some(result); // adding the result in each monitor
    }

    // this will handle more errors based on multiple wrong user input cases
    if let Some(err) = check_error_cases(&args[1..]).err() {
        println!("💥💥 Error in input {}", err);
        return Ok(());
    }

    println!("Monitors successfully written to file.");
    println!("🏃‍♂️ Running process Monitors...");

    // Clone the Arc<Mutex<Monitors>> for each task
    let monitors1 = Arc::clone(&monitors);
    let monitors2 = Arc::clone(&monitors);

    // Spawn two tasks to run the async functions concurrently
    let task1 = spawn(update_monitors(monitors1));
    let task2 = spawn(store_monitors(monitors2));

    // Wait for both tasks to complete
    task1.await?;
    task2.await?;

    Ok(())
}

async fn update_monitors(monitors: Arc<Mutex<Monitors>>) {
    let mut interval = interval(Duration::from_secs(3));

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
            println!("Monitors successfully written to file.");
        }
    }
}

async fn store_monitors(monitors: Arc<Mutex<Monitors>>) {
    let mut interval = interval(Duration::from_secs(1));

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
            println!("Monitors successfully written to file.");
        }
    }
}
