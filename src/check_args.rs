use std::env;
use std::process::{Command, exit};
use std::str;
use std::fs;

fn run_python_program(num: &str) -> String {
    let output = Command::new("python2.7")
        .arg("npuzzle-gen.py")
        .arg(num)
        .output();

    match output {
        Ok(output) => {
            if output.status.success() {
                let output_str = str::from_utf8(&output.stdout).unwrap();
                output_str.to_string()
            } else {
                println!("Python script returned an error: {}", str::from_utf8(&output.stderr).unwrap());
                exit(1);
            }
        },
        Err(e) => {
            println!("Failed to execute command: {}", e);
            exit(1);
        }
    }
}

fn get_file_contents(file: &str) -> String {
    let contents = match fs::read_to_string(file) {
        Ok(contents) => contents,
        Err(_) => {
            println!("Error: File not found or not readable.");
            exit(1);
        }
    };
    // println!("contents: {}", contents);
    contents
}

pub fn check_args() -> (String, String, String, i16) {
    let mut file = String::new();
    let mut filefound: bool = false;
    let mut method: String = "greedy".to_string();
    let mut methodfound: bool = false;
    let mut heuristic: String = "manhattan".to_string();
    let mut heuristicfound: bool = false;
    let mut o_value: i16 = -1;
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    // println!("{:?}", args);
    // println!("{}", file);
    while i < args.len() {
        // println!("{}, {}, {}", args[i], i, args.len());
        if args[i] == "-f" || args[i] == "--file" {
            if i + 1 < args.len()  && filefound == false {
                filefound = true;
                file = get_file_contents(&args[i + 1]);
                // println!("file content:\n{}", file);
            } else {
                println!("File flag detected but no argument following it. Exiting.");
                exit(1);
            }
        }
        else if args[i] == "-g" || args[i] == "--generate" {
            if i + 1 < args.len() && filefound == false  && args[i + 1].parse::<i8>().is_ok() {
                file = run_python_program(&args[i + 1]);
                filefound = true;
                // println!("Python output:\n{}", file);
            } else {
                println!("File flag detected but no integer argument following it. Exiting.");
                exit(1);
            }
        }
        else if args[i] == "-he" || args[i] == "--heuristic" {
            if i + 1 < args.len() && heuristicfound == false && (args[i + 1] == "manhattan" || args[i + 1] == "hamming" || args[i + 1] == "euclidean") {
                heuristicfound = true;
                heuristic = args[i + 1].clone();
            } else {
                println!("File flag detected but no argument following it. Exiting.");
                exit(1);
            }
        }
        else if args[i] == "-m" || args[i] == "--method" {
            if i + 1 < args.len() && methodfound == false && (args[i + 1] == "greedy" || args[i + 1] == "uniform")  {
                methodfound = true;
                method = args[i + 1].clone();
            } else {
                println!("File flag detected but no argument following it. Exiting.");
                exit(1);
            }

        }
        else if args[i] == "-o" || args[i] == "--override"{
            if i + 1 < args.len() && o_value == -1 && (args[i + 1].parse::<i16>().is_ok()) {
                o_value = args[i + 1].parse::<i16>().unwrap();
                if o_value < 0 {
                    println!("Override flag detected but no positive integer argument following it. Exiting.");
                    exit(1);
                }
            } else if o_value != -1 {
                println!("Override flag detected more than once. Exiting.");
            }
            else {
                o_value = -2;
                i+=1;
                continue;
            }
        }
        else if i == 1 && (args[i] == "-h" || args[i] == "--help") {
            println!("Usage: cargo run -- -f | -g | -m | -he\n
            [-f | --file] [file]\n
            [-g | --generate] [size]\n
            [-m | --method] [method]\n
            [-he | --heuristic] [heuristic]");
            exit(1);
        }
        else {
            println!("Invalid argument detected. Exiting. Run with -h or --help for usage.");
            exit(1);
        }
        i += 2;
    }
    if args.len() == 1 {
        println!("No arguments detected. Exiting. Run \"Cargo run -- -h\" or \"Cargo run -- --help\" for usage or check the README.md.");
        exit(1);
    }
    if filefound == false {
        println!("No file or generation flag detected. Exiting. Run \"Cargo run -- -h\" for more info.");
        exit(1);
    }
    println!("o_value: {}", o_value);
    // println!("file: {}, method: {}, heuristic: {}", file, method, heuristic);
    return (file, method, heuristic, o_value);
}