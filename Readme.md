# Process Monitor

## Overview

This Rust program, `process_monitor`, is designed to read monitor data from a specified JSON file, generate random numeric values for each monitor, and print the results. The program takes command-line arguments to specify the path to the monitor file.

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
