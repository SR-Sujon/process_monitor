use rand::Rng;
use serde::{Serialize, Serializer};
use serde_json::{to_writer, Value, from_str, Result as JsonResult};
use std::{env, fs, time::SystemTime, fs::File};

#[derive(Debug, Serialize)]
struct ResultValSys {
    value: i32,
    processed_at: SystemTime,
}

impl ResultValSys {
    fn new(value: i32) -> Self {
        Self {
            value,
            processed_at: SystemTime::now(),
        }
    }
}

fn process_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 || !args[2].starts_with("-monitorFile") {
        return Err("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file".to_string());
    }
    Ok(args[3].clone())
}

fn read_monitors_file(monitor_file_path: &str) -> Result<Value, String> {
    fs::read_to_string(monitor_file_path)
        .map_err(|e| format!("Failed to read file: {}", e))
        .and_then(|content| from_str(&content).map_err(|e| format!("Failed to parse JSON: {}", e)))
}

fn main() {
    let monitor_file_path = match process_args() {
        Ok(path) => path,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    let monitors = match read_monitors_file(&monitor_file_path) {
        Ok(monitors) => monitors,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };
    println!("Monitors: {}", monitors);

    // Creates a random number generator
    let mut rng = rand::thread_rng();

    // Access the "monitors" array from the JSON data
    if let Some(monitors_array) = monitors["monitors"].as_array() {
        let mut results: Vec<ResultValSys> = Vec::new();

        for monitor in monitors_array {
            // Extract relevant fields
            let name = monitor["name"].as_str().unwrap_or("Unnamed Monitor");
            let code = monitor["code"].as_str().unwrap_or("No Code");

            // Generate a random numeric value
            let random_value: i32 = rng.gen_range(1..100);

            // Create an instance of ResultValSys with random values
            let result = ResultValSys::new(random_value);

            // Print the monitor details and result
            println!("Monitor: {}, Code: {}", name, code);
            println!("{:?}", result);

            // Collect results for writing to JSON file
            results.push(result);
        }

        // Write results to a new JSON file
        if let Ok(file) = File::create("output.json") {
            if let Err(err) = to_writer(file, &results) {
                eprintln!("Failed to write to JSON file: {}", err);
            } else {
                println!("Results written to 'output.json'.");
            }
        } else {
            eprintln!("Failed to create or open output.json file.");
        }
    } else {
        eprintln!("Invalid monitors.json format: 'monitors' array not found.");
    }
}
