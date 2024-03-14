use serde::{Deserialize, Serialize};
use std::{env, fs, time::SystemTime};
use chrono::{Utc, Duration};
use rand::Rng;

#[derive(Debug, Serialize, Deserialize)]
struct ResUpdate {
    value: i32,
    processed_at: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Monitor {
    name: String,
    script: Option<String>,
    result: Option<String>,
    code: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Monitors {
    monitors: Vec<Monitor>,
}

fn process_monitor(monitor_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = match fs::read_to_string(monitor_file) {
        Ok(data) => data,
        Err(e) => return Err(Box::new(e)),
    };

    let mut monitors: Monitors = match serde_json::from_str(&json_data) {
        Ok(data) => data,
        Err(e) => return Err(Box::new(e)),
    };

    update_monitors(&mut monitors);
    store_monitors(&monitors, get_timestamp());

    let json_data = match serde_json::to_string(&monitors) {
        Ok(data) => data,
        Err(e) => return Err(Box::new(e)),
    };
    fs::write(monitor_file, json_data)?;

    Ok(())

}



fn update_monitors(monitors: &mut Monitors) {
    for monitor in &mut monitors.monitors {
        let result = ResUpdate {
            value: rand::thread_rng().gen_range(0..100),
            processed_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        };

        monitor.result = Some(serde_json::to_string(&result).unwrap());
    }
}

fn store_monitors(monitors: &Monitors, timestamp: String) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = format!("{}_{}.json", timestamp, "monitors");
    let file_path = format!("./{}", file_name);
    let json_data = match serde_json::to_string(monitors) {
        Ok(data) => data,
        Err(e) => return Err(Box::new(e)),
    };
    let fpath = file_path.clone();
    fs::write(file_path, json_data)?;
    println!("Stored monitors in: {}", fpath);

    Ok(())
}

fn get_timestamp() -> String {
    let now = Utc::now();
    format!("{}", now.format("%Y-%m-%d_%H-%M"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("{}",args.len());
    if args.len() != 4 {
        eprintln!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
std::process::exit(1);
    }

    let monitor_file = &args[3];
    process_monitor(monitor_file)?;

    Ok(())
}