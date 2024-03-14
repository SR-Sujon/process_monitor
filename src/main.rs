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
/* 
fn process_monitor(monitor_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(monitor_file)?;
    let mut monitors: Monitors = serde_json::from_str(&json_data)?;

    for monitor in &mut monitors.monitors {
        println!("Name: {}", monitor.name);
        println!("Script: {:?}", monitor.script);
        println!("Result: {:?}", monitor.result);
        println!("Code: {}", monitor.code);
        println!();

        let result = ResUpdate {
            value: rand::random::<i32>(),
            processed_at: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs(),
        };

        monitor.result = Some(serde_json::to_string(&result)?);
    }

    let json_data = serde_json::to_string(&monitors)?;
    fs::write(monitor_file, json_data)?;

    Ok(())
}
*/

fn process_monitor(monitor_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let json_data = fs::read_to_string(monitor_file)?;
    let mut monitors: Monitors = serde_json::from_str(&json_data)?;

    update_monitors(&mut monitors);
    store_monitors(&monitors, get_timestamp());

    let json_data = serde_json::to_string(&monitors)?;
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
    let json_data = serde_json::to_string(monitors)?;
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