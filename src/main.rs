mod idastar;
mod map;
use map::Map;
mod parser;
mod check_args;
use check_args::check_args;

fn main() {
    let map = Map {
        number: 5,
        matrix: vec![
            vec![0, 5, 8],
            vec![4, 1, 7],
            vec![3, 2, 6],
        ],
    };

    // jareste
    let (file, method, heuristic) = check_args();

    let mut parser: Option<parser::Parser> = None;
    match parser::Parser::parse_file(file) {
        Ok(p) => {
            parser = Some(p);
        }
        Err(error) => {
            println!("Failed to parse file: {:?}", error);
        }
    }
    
    // debug veure q parser existeix i printa algo
    if let Some(parser) = parser {
        println!("parser.map:\n{:?}", parser.map);
    }
    // debug veure q parser existeix i printa algo
    // jareste

    // idastar::idastar(map);

}


// fn main() {
//     // let args: Vec<String> = env::args().collect();
//     // for i in 0..args.len() {
//     //     println!("{}", args[i]);
//     //     if args[i] == "-f" || args[i] == "--file" {
//     //         if i + 1 < args.len() {
//     //             println!("File flag detected, next argument is: {}", args[i + 1]);
//     //         } else {
//     //             println!("File flag detected but no argument following it. Exiting.");
//     //             exit(1);
//     //         }
//     //     }
//     // }
//     check_args();
//     let filename = "test.txt";

//     let mut parser: Option<parser::Parser> = None;
//     // check_args();
//     // let mut parser: Option<parser::Parser> = None;

//     match parser::Parser::parse_file(filename) {
//         Ok(p) => {
//             parser = Some(p);
//         }
//         Err(error) => {
//             println!("Failed to parse file: {:?}", error);
//         }
//     }
//     // match parser::Parser::parse_file(check_args()) {
//     //     Ok(p) => {
//     //         parser = Some(p);
//     //     }
//     //     Err(error) => {
//     //         println!("Failed to parse file: {:?}", error);
//     //     }
//     // }

//     if let Some(mut parser) = parser {
//         parser.map[0][0] = 10;
//         println!("Parser.map[0][0]: {}", parser.map[0][0]);
//     }
//     // if let Some(mut parser) = parser {
//     //     parser.map[0][0] = 10;
//     //     println!("Parser.map[0][0]: {}", parser.map[0][0]);
//     // }
// }