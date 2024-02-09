use std::env;
use std::process::{Command, exit};
use std::str;

fn run_python_program(num: &str) -> String {
    let output = Command::new("python")
        .arg("npuzzle-gen.py")
        .arg(num)
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let output_str = str::from_utf8(&output.stdout).unwrap();
        output_str.to_string()
    } else {
        println!("Python script returned an error: {}", str::from_utf8(&output.stderr).unwrap());
        exit(1);
    }
}

pub fn check_args() {
    let mut method: &str = "greedy";
    let mut methodfound: bool = false;
    let mut heuristic: &str = "manhattan";
    let mut heuristicfound: bool = false;
    let mut file = String::new();
    let mut filefound: bool = false;
    let args: Vec<String> = env::args().collect();
    for i in 0..args.len() {
        // println!("{}, {}, {}", args[i], i, args.len());
        if args[i] == "-f" || args[i] == "--file" {
            if i + 1 < args.len() {
                println!("File flag detected, next argument is: {}", args[i + 1]);
            } else {
                println!("File flag detected but no argument following it. Exiting.");
                exit(1);
            }
        }
        if args[i] == "-g" || args[i] == "--generate" {
            if i + 1 < args.len() {
                println!("Generate flag detected, next argument is: {}", args[i + 1]);
                let python_output = run_python_program(&args[i + 1]);
                println!("Python output: {}", python_output);
            } else {
                println!("File flag detected but no argument following it. Exiting.");
                exit(1);
            }
        }
        if args[i] == "-he" || args[i] == "--file" {
            if i + 1 < args.len() {
                println!("File flag detected, next argument is: {}", args[i + 1]);
            } else {
                println!("File flag detected but no argument following it. Exiting.");
                exit(1);
            }
        }
    }
}