# Process Monitoring System
This is a simple monitoring system written in Rust. It reads a JSON file containing a list of monitors, generates random results for each monitor, and saves the updated monitors back to the JSON file.
// Copyright (c) 2024


## Usage

To use the program, run the following command in your terminal:

```bash
$ process_monitor -monitorFile /path/to/given/monitors.json/file
Replace /path/to/given/monitors.json/file with the actual path to your monitors JSON file.
```
## Command Line arguments
-monitorFile: Specifies the path to the monitors JSON file.

## Structure
The code is structured as follows:

-ResultValSys struct: Represents the result of a monitor with a randomly generated value and the timestamp of when it was processed.

-process_args function: Processes command-line arguments and returns the path to the monitor file or an error message.

-read_monitors_file function: Reads the content of the monitor file, parses it as JSON, and returns the parsed data or an error message.

-main function: The main entry point of the program. It orchestrates the processing of command-line arguments, reading the monitor file, generating random values, and printing the results.

## Dependencies:

The program relies on the following external libraries:

rand: Used for generating random numbers.
serde_json: Used for parsing JSON data.

## How to Build
Ensure that you have Rust installed on your system. Then, navigate to the project directory and run:
```bash
$ cargo build
```
This will compile the program.

## How to Run
```bash
$ cargo run -- -monitorFile /path/to/given/monitors.json/file
```
## Example Output
The program will print details for each monitor, including its name, code, and the randomly generated result, along with the timestamp when it was processed.

## Error Handling
The program provides informative error messages in case of incorrect command-line arguments or issues with reading/parsing the monitor file.

## Functions
### fn update_monitors(monitors: &mut Monitors)
This function updates the result field of each monitor in the monitors slice with a random value and the current timestamp.

1. Iterate through each monitor in the monitors slice.
2. Generate a random value between 0 and 99.
3. Get the current timestamp as a Unix 4. timestamp in seconds.
5. Create a ResUpdate struct with the random value and the timestamp.
6. Serialize the ResUpdate struct to a JSON string and store it in the result field of the monitor.

### fn store_monitors(monitors: &Monitors, timestamp: String) -> Result<(), Box<dyn std::error::Error>>
This function saves the updated monitors to a new JSON file with a timestamp in the filename.

1. Create a filename with the current timestamp and the string "monitors".
2. Serialize the monitors struct to a JSON string.
3. Write the JSON string to a new file with the created filename.
4. Print the path of the saved file to the console.

### fn get_timestamp() -> String
This function returns the current timestamp as a string in the format "YYYY-MM-DD_HH-MM".

1. Get the current time in UTC.
2. Format the time as a string with the format "%Y-%m-%d_%H-%M".
3. Return the formatted string.


### fn main() -> Result<(), Box<dyn std::error::Error>>

This is the main function that runs the monitoring system.

1. Get the command line arguments and check if the correct number of arguments are provided.
2. If the correct number of arguments are not provided, print the usage message and exit with a non-zero status code.
3. Get the path to the monitors JSON file from the command line arguments.
4. Read the monitors JSON file and deserialize it to aMonitors struct.
5. Call the update_monitors function to update the monitors.
6. Call the store_monitors function to save the updated monitors to a new JSON file.
7. Serialize the monitors struct to a JSON string and overwrite the original monitors JSON file.
8. Return a successful result.

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
cargo run -- -monitorFile /path/to/monitors.json
```

The monitoring system will update the result field of each monitor with a random value and the current timestamp, and save the updated monitors to a new JSON file with a timestamp in the filename. The original monitors JSON file will also be overwritten with the updated monitors.