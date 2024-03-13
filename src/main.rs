use rand::Rng;
use serde_json::{Value, from_str};
use std::{env, fs, time::SystemTime};

#[derive(Debug)]
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
        }
    } else {
        eprintln!("Invalid monitors.json format: 'monitors' array not found.");
    }
}
