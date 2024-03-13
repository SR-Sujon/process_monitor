use rand::Rng;
use serde_json::{Error, Value};
use std::env;
use std::fs;
use std::time::{SystemTime,UNIX_EPOCH};

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
    if args.len() != 3 {
        return Err(
            "Usage: process_monitor monitorFile /path/to/given/monitors.json/file".to_string(),
        );
    }
    let monitor_file_path = &args[1];
    if !monitor_file_path.starts_with("monitorFile") {
        return Err("Invalid argument. Usage: process_monitor -monitorFile /path/to/given/monitors.json/file".to_string());
    }
    let monitor_file_path = &args[2];
    Ok(monitor_file_path.to_string())
}

fn read_monitors_file(monitor_file_path: &str) -> Result<Value, String> {
    let file_content =
        fs::read_to_string(monitor_file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    let monitors: Value =
        serde_json::from_str(&file_content).map_err(|e| format!("Failed to parse JSON: {}", e))?;
    Ok(monitors)
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

    // The value field is an integer representing the result value, and the processed_at field is a SystemTime representing the time the result was processed.
    //let result = ResultValSys::new(42);
    //println!("{:?}", result);


    // Create a random number generator
    let mut rng = rand::thread_rng();

    // Create an instance to hold the JSON data
    let monitors_data = match read_monitors_file(&monitor_file_path) {
        Ok(monitors) => monitors,
        Err(err) => {
            eprintln!("{}", err);
            return;
        }
    };

    // Access the "monitors" array from the JSON data
    if let Some(monitors_array) = monitors_data["monitors"].as_array() {
        for monitor in monitors_array {
            // Extract relevant fields
            let name = monitor["name"].as_str().unwrap_or("Unnamed Monitor");
            let code = monitor["code"].as_str().unwrap_or("No Code");

            // Generate a random numeric value
            let random_value: i32 = rng.gen_range(1..100);

            // Get current time in seconds
            let processed_at:SystemTime  = SystemTime::now();
            
            // Create an instance of ResultValSys with random values
            let result = ResultValSys {
                value: random_value,
                processed_at,
            };

            // Print the monitor details and result
            println!("Monitor: {}, Code: {}", name, code);
            println!("{:?}", result);
        }
    } else {
        eprintln!("Invalid monitors.json format: 'monitors' array not found.");
    }
}
