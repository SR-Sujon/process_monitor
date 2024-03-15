# Process Monitoring System
This is a simple monitoring system written in Rust. It reads a JSON file containing a list of monitors, generates random results for each monitor, and saves the updated monitors back to the JSON file.
// Copyright (c) 2024


## Overview

### Data Structures:
* ResUpdate: This structure represents an update to the monitoring data. It consists of two fields:

2. value: An integer representing the value of the update.
3. processed_at: A Unix timestamp indicating when the update was processed.
* Monitor: This structure represents a monitoring entity. It contains the following fields:

1. name: A string representing the name of the monitor.
2. script: An optional string containing the script associated with the monitor.
3. result: An optional string representing the result of the monitor.
4. code: A string containing the code associated with the monitor.
* Monitors: This structure holds a collection of Monitor instances in a vector.

### Processing:
1. process_monitor(): This function is responsible for reading monitoring data from a JSON file specified via a command-line argument. It then spawns two threads:
- An update thread that periodically updates the monitoring data.
- A store thread that periodically stores the monitoring data to disk.
After the threads complete their tasks, the updated monitoring data is serialized back to JSON and written to the input file.

### Updating Monitors
* update_monitors(): This function generates random update values for each monitor and updates the result field with the generated value and timestamp. It is called periodically by the update thread.

### Storing Monitors:
- store_monitors(): This function serializes the monitoring data to JSON and writes it to a new file in a logs directory. The filename includes a timestamp indicating when the data was stored. It is called periodically by the store thread.

### Main Function:
- main(): This function serves as the entry point of the program. It parses command-line arguments to determine the input file path. Then it runs the process_monitor function in a loop for 1 minute, with a 5-minute sleep between iterations.

### Output
- Throughout execution, the program prints messages indicating when it invokes the process_monitor function, updates to monitors, and storage of monitors. These messages provide feedback to the user about the program's progress and activities.


## Usage
To use the program, run the following command in your terminal:

```bash
$ process_monitor -monitorFile /path/to/given/monitors.json/file
Replace /path/to/given/monitors.json/file with the actual path to your monitors JSON file.
```
## Command Line arguments
-monitorFile: Specifies the path to the monitors JSON file.


## Dependencies:

The program relies on the following external libraries:

serde_json = "1.0"
rand = "0.8"
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
chrono = "0.4"
async-std = "1.10.0"

## How to Build
Ensure that you have Rust installed on your system. Then, navigate to the project directory and run:
```bash
$ cargo build
```
This will compile the program.

## How to Run
```bash
cargo run process_monitor -monitorFile ./assets/monitors.json
```
## Example Output
The program will print details for each monitor, including its name, code, and the randomly generated result, along with the timestamp when it was processed.

## Error Handling
The program provides informative error messages in case of incorrect command-line arguments or issues with reading/parsing the monitor file.


## Documentation
To use this monitoring system, you need to provide a JSON file with a list of monitors. The JSON file should look like this:

{
  "monitors": [
    {
      "name": "Monitor 1",
      "script": "some script",
      "result": null,
      "code": "some code"
    },
    {
      "name": "Monitor 2",
      "script": "some script",
      "result": null,
      "code": "some code"
    }
  ]
}


You can then run the monitoring system with the following command:
```bash
cargo run process_monitor -monitorFile ./assets/monitors.json
```

The monitoring system will update the result field of each monitor with a random value and the current timestamp, and save the updated monitors to a new JSON file with a timestamp in the filename. The original monitors JSON file will also be overwritten with the updated monitors.