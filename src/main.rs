mod idastar;
mod map;
use map::Map;
mod parser;
mod check_args;
use check_args::check_args;
mod generate_goal;
use generate_goal::generate_goal;
use std::process::exit;
mod check_solution;
use check_solution::check_solution;


fn main() {
    // jareste
    let (file, method, heuristic) = check_args();
    
    let mut matrix: Option<Vec<Vec<i16>>> = None;
    match parser::parse_file(file) {
        Ok(m) => {
            matrix = Some(m);
        }
        Err(error) => {
            println!("Failed to parse file: {:?}", error);
            exit(1);
        }
    }

    if let Some(matrix) = matrix {
        println!("In matrix:");
        for i in 0..matrix[0].len() {
            println!("{:?}", matrix[i]);
        }
        if check_solution(&matrix) == false {
            println!("The input is not a valid solution");
            exit(1);
        }
        generate_goal(matrix[0].len());

    }
    // debug veure q parser existeix i printa algo
    // jareste

    // idastar::idastar(map);

}
