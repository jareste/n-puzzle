mod idastar;
mod map;
use map::Map;
mod parser;
mod check_args;
use check_args::check_args;
mod generate_goal;
use generate_goal::generate_goal;
use std::process::exit;


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
        // println!("matrix:\n{:?}", matrix);
        generate_goal(matrix[0].len());
    }
    // debug veure q parser existeix i printa algo
    // jareste

    // idastar::idastar(map);

}
