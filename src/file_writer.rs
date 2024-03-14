use serde_json::to_string_pretty;
use std::fs::File;
use std::io::{BufWriter, Write};

use crate::monitors::Monitors;

pub fn write_to_file(monitors: &Monitors, file_path: &str) -> Result<(), std::io::Error> {
    // serialize the monitors to JSON
    let new_json: String = to_string_pretty(&monitors)?;

    // write the JSON string to a file
    let mut file: BufWriter<File> = BufWriter::new(File::create(file_path)?);
    // write the json to the specified file path
    file.write_all(new_json.as_bytes())?;

    Ok(())
}
