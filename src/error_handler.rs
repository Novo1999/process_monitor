use anyhow::Result;

// constants
const MAX_LENGTH: i32 = 3;
pub const USAGE_COMMAND: &str =
    "! Usage: process_monitor -monitorFile /path/to/given/monitors.json/file";
const EXECUTABLE_NAME: &str = "process_monitor";
const MONITOR_FILE_COMMAND: &str = "-monitorFile";

pub fn check_error_cases(sliced_args: &[String]) -> Result<()> {
    // Check if the argument count is correct
    if sliced_args.len() < MAX_LENGTH as usize {
        println!("{}", "💥💥 Too few arguments!");
        return Ok(());
    }

    if sliced_args.len() > MAX_LENGTH as usize {
        println!("{}", "💥💥 Too many arguments!");
        return Ok(());
    }

    println!("{:?}", sliced_args);

    if sliced_args.len() == 1 {
        println!("{}", "💥💥 Too few arguments!");
        return Ok(());
    }

    // if first arg is not process_monitor show error
    if sliced_args[0] != EXECUTABLE_NAME {
        println!(
            "💥💥 Wrong executable file --> ({}) {}",
            sliced_args[0], USAGE_COMMAND
        );
        return Ok(());
    }
    // if second arg is not -monitorFile show error
    if sliced_args[1] != MONITOR_FILE_COMMAND {
        println!(
            "💥💥 Wrong command --> ({}) {}",
            sliced_args[1], USAGE_COMMAND
        );
        return Ok(());
    }
    Ok(())
}
