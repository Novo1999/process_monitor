use crate::error_handler::USAGE_COMMAND;
use anyhow::{Context, Result as AnyhowResult};
use error_handler::check_error_cases;
use monitors::Monitors;
use rand::Rng;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tokio::spawn;

// module declarations
mod error_handler;
mod file_writer;
mod monitors;

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

    process_monitors(monitors).await;

    Ok(())
}

async fn process_monitors(monitors: Arc<Mutex<Monitors>>) {
    // Clone the Monitors
    let monitors1 = Arc::clone(&monitors);
    let monitors2 = Arc::clone(&monitors);

    // Spawn two tasks to run the async functions concurrently
    let task1 = spawn(monitors::update_monitors(monitors1));
    let task2 = spawn(monitors::store_monitors(monitors2));

    // Wait for both tasks to complete or until 5 minutes have passed
    tokio::time::sleep(Duration::from_secs(300)).await;

    // Cancel the tasks
    task1.abort();
    task2.abort();
    println!("⌚ 5 Minutes have passed, Monitors closed");
}
