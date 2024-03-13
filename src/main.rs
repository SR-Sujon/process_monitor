use std::env;
use std::fs;
use serde_json::{Value, Error};

fn process_args() -> Result<String, String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        return Err("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file".to_string());
    }
    let monitor_file_path = &args[1];
    if !monitor_file_path.starts_with("monitorFile") {
        return Err("Invalid argument. Usage: process_monitor -monitorFile /path/to/given/monitors.json/file".to_string());
    }
    let monitor_file_path = &args[2];
    Ok(monitor_file_path.to_string())
}

fn read_monitors_file(monitor_file_path: &str) -> Result<Value, String> {
    let file_content = fs::read_to_string(monitor_file_path).map_err(|e| format!("Failed to read file: {}", e))?;
    let monitors: Value = serde_json::from_str(&file_content).map_err(|e| format!("Failed to parse JSON: {}", e))?;
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
}