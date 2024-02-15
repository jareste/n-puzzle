use std::env;
use std::process::{Command, exit};
use std::str;
use std::fs;

fn run_python_program(num: &str) -> String {
    let output = Command::new("python")
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

pub fn check_args() -> (String, String, String, i16, String) {
    let mut file = String::new();
    let mut filefound: bool = false;
    let mut method: String = "normal".to_string();
    let mut methodfound: bool = false;
    let mut heuristic: String = "manhattan".to_string();
    let mut heuristicfound: bool = false;
    let mut algorithm: String = "ida_star".to_string();
    let mut algorithmfound: bool = false;
    let mut o_value: i16 = -1;
    let args: Vec<String> = env::args().collect();
    let mut i = 1;
    while i < args.len() {
        if args[i] == "-f" || args[i] == "--file" {
            if i + 1 < args.len()  && filefound == false {
                filefound = true;
                file = get_file_contents(&args[i + 1]);
            } else {
                println!("File flag detected but no argument following it. Exiting.");
                exit(1);
            }
        }
        else if args[i] == "-g" || args[i] == "--generate" {
            if i + 1 < args.len() && filefound == false  && args[i + 1].parse::<i8>().is_ok() {
                file = run_python_program(&args[i + 1]);
                filefound = true;
            } else {
                println!("File flag detected but no integer argument following it. Exiting.");
                exit(1);
            }
        }
        else if args[i] == "-he" || args[i] == "--heuristic" {
            if i + 1 < args.len() && heuristicfound == false && (args[i + 1].to_lowercase() == "manhattan" || args[i + 1].to_lowercase() == "hamming" || args[i + 1].to_lowercase() == "euclidean" || args[i + 1].to_lowercase() == "linear_conflicts" || args[i + 1].to_lowercase() == "noadmisible") {
                heuristicfound = true;
                heuristic = args[i + 1].clone();
            } else {
                println!("File flag detected but no argument following it. Exiting.");
                exit(1);
            }
        }
        else if args[i] == "-m" || args[i] == "--method" {
            if i + 1 < args.len() && methodfound == false && (args[i + 1].to_lowercase() == "greedy" || args[i + 1].to_lowercase() == "uniform" || args[i + 1].to_lowercase() == "normal")  {
                methodfound = true;
                method = args[i + 1].clone();
            } else {
                println!("Method flag detected but no valid argument following it. Exiting.");
                exit(1);
            }

        }
        else if args[i] == "-o" || args[i] == "--override"{
            if o_value != -1 {
                println!("Override flag detected more than once. Exiting.");
            }
            else {
                o_value = -2;
                i+=1;
                continue;
            }
        }
        else if args[i] == "-a" || args[i] == "--algorithm" {
            if i + 1 < args.len() && (args[i + 1].to_lowercase() == "a_star" || args[i + 1].to_lowercase() == "ida_star") && algorithmfound == false {
                algorithmfound = true;
                algorithm = args[i + 1].clone();
            } else {
                println!("Algorithm flag detected but no argument following it. Exiting.");
                exit(1);
            }
        }
        else if i == 1 && (args[i] == "-h" || args[i] == "--help") {
            println!("Usage: cargo run -- [-f FILENAME] [-g SIZE] [-a ALGORITHM] [-m METHOD] [-he HEURISTIC] [-o MOVEMENTS]\n
            [-f | --file] [filename]\n
            [-g | --generate] [size]\n
            [-a | --algorithm] [a_star | ida_star]
            a_star: Faster algorithm but requires more memory and may lead to a notable slowness of the computer.
            ida_star: Slower algorithm based on astar that does not requires that much memory but takes longer time to get the solution.\n
            [-m | --method] [normal | greedy | uniform]
            normal: knows the previous steps and the cost of the path.
            greedy: only knows the cost of the path.
            uniform: only knows the previous steps.\n
            [-he | --heuristic]
            manhattan: sum of the distances of the tiles to their goal positions.
            hamming: number of tiles in the wrong position.
            euclidean: sum of the squares of the distances of the tiles to their goal positions.
            linear_conflicts: sum of the manhattan distances and the number of conflicts.
            NoAdmisible: twice the manhattan distance.\n
            [-o | --override] 
            This flag allows the execution of 5 or higher puzzle sizes.\n
            By defaut, the program will run with the following parameters: ida_star, normal, manhattan.
            Also, for protecting computer resources, the program will not allow higher than 4x4 puzzles without specifying the maximum number of movements to check (-o).");
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
    return (file, method, heuristic, o_value, algorithm);
}