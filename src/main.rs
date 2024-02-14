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
use crate::map::Map;


fn main() {
    // jareste
    let (file, method, heuristic, o_value) = check_args();
    
    let mut map: Option<Map> = None;
    match parser::parse_file(file) {
        Ok(m) => {
            map = Some(m);
        }
        Err(error) => {
            println!("Failed to parse file: {:?}", error);
            exit(1);
        }
    }

    if let Some(map) = map {
        println!("In matrix:");
        for i in 0..map.matrix[0].len() {
            println!("{:?}", map.matrix[i]);
        }
        if check_solution(&map.matrix) == false {
            println!("The input is not a valid solution");
            exit(1);
        }
        let goal = generate_goal(map.matrix[0].len());
        // let goal2 = goal.clone();
        // println!("map.x: {}", map.x);
        // println!("map.y: {}", map.y);
        // println!("map 0: {}", map.matrix[map.x as usize][map.y as usize]);
        solver(map, goal);

    }
    //debug veure q parser existeix i printa algo
    // jareste

}
