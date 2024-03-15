use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    sync::{Arc, Mutex},
    thread,
    time::{Duration, SystemTime},
};
use chrono::{Utc};
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
    let monitors: Monitors = match serde_json::from_str(&json_data) {
        Ok(data) => data,
        Err(e) => return Err(Box::new(e)),
    };
    
    let monitors_arc = Arc::new(Mutex::new(monitors));
    let update_monitors_arc = Arc::clone(&monitors_arc);
    let store_monitors_arc = Arc::clone(&monitors_arc);

    let update_thread = thread::spawn(move || {
        for _ in 0..10 {
            update_monitors(&update_monitors_arc);
            println!("update_monitors_invoked");
            thread::sleep(Duration::from_secs(30));
        }
    });

    let store_thread = thread::spawn(move || {
        for _ in 0..5 {
            store_monitors(&store_monitors_arc, get_timestamp());
            println!("store_monitors_invoked");
            thread::sleep(Duration::from_secs(60));
        }
    });

    update_thread.join().unwrap();
    store_thread.join().unwrap();

    let monitors_guard = monitors_arc.lock().unwrap();
    let json_data = match serde_json::to_string(&*monitors_guard) {
        Ok(data) => data,
        Err(e) => return Err(Box::new(e)),
    };
    fs::write(monitor_file, json_data)?;

    Ok(())
}

fn update_monitors(monitors: &Arc<Mutex<Monitors>>) {
    let mut monitors = monitors.lock().unwrap();
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
    println!("Monitors updated at {}", get_timestamp());
}

fn store_monitors(monitors: &Arc<Mutex<Monitors>>, timestamp: String) {
    let file_name = format!("{}_{}.json", timestamp, "monitors");
    let file_path = format!("./logs/{}", file_name);
    let mut monitors = monitors.lock().unwrap();
    let json_data = match serde_json::to_string(&*monitors) {
        Ok(data) => data,
        Err(e) => {
            eprintln!("Error serializing monitors: {}", e);
            return;
        }
    };
    if let Err(e) = fs::write(&file_path, json_data) {
        eprintln!("Error writing monitors to file: {}", e);
        return;
    }
    println!("Stored monitors in: {}", file_path);
}

fn get_timestamp() -> String {
    let now = Utc::now();
    format!("{}", now.format("%Y-%m-%d_%H-%M"))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    println!("Parameters len: {}", args.len());
    if args.len() != 4 {
        eprintln!("Usage: process_monitor -monitorFile /path/to/given/monitors.json/file");
        std::process::exit(1);
    }

    let monitor_file = &args[3];
    // Run the loop for 1 minute
    for _ in 0..1 {
        println!("process_monitor_invoked...");
        process_monitor(monitor_file)?;
        
        // Sleep for 30 seconds
        std::thread::sleep(Duration::from_secs(300));
    }
    println!("process_monitor_terminated successfully.");

    Ok(())
}
