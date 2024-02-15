mod map;
mod solver;
mod astar;
mod idastar;
use solver::solver;
mod parser;
mod check_args;
use check_args::check_args;
mod generate_goal;
use generate_goal::generate_goal;
use std::process::exit;
mod check_solution;
use check_solution::check_solution;

fn main() {
    let (file, method, heuristic, o_value, algorithm) = check_args();
    match parser::parse_file(file) {
        Ok(map) => {
            if check_solution(&map.matrix) == false {
                println!("The input is not a valid solution");
                exit(1);
            }
            else{
                if o_value == -1 && map.size > 4 {
                    println!("For safety reasons, the program will not run with a map size greater than 4 without an o_value. Exiting.");
                    exit(1);
                }
                else if o_value == -2 {
                    solver(&heuristic, &algorithm,&method, &map, &generate_goal(map.size as usize), usize::MAX);
                }
                else {
                    let goal = generate_goal(map.size as usize);
                    solver(&heuristic, &algorithm,&method, &map, &goal, usize::MAX);
                }
            }
        }
        Err(error) => {
            println!("Failed to parse file: {:?}", error);
            exit(1);
        }
    }
}
