use rand::Rng;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time::{interval, Duration as TokyoDuration};

use crate::{
    file_writer::write_to_file,
    monitors::{self, Monitors},
};

#[tokio::main]
pub async fn update_monitors(mut monitors: Monitors) {
    let mut interval = interval(TokyoDuration::from_secs(2));

    loop {
        interval.tick().await;
        println!("Running update monitors");

        for monitor in &mut monitors.monitors {
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

        // Write monitors to JSON file
        if let Err(err) = write_to_file(&monitors, "assets/monitors_with_result.json") {
            println!("Failed to write monitors to file: {}", err);
        } else {
            println!("Monitors successfully written to file.");
        }
    }
}
