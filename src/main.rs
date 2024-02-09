mod parser;
mod check_args;
use check_args::check_args;

fn main() {
    // let args: Vec<String> = env::args().collect();
    // for i in 0..args.len() {
    //     println!("{}", args[i]);
    //     if args[i] == "-f" || args[i] == "--file" {
    //         if i + 1 < args.len() {
    //             println!("File flag detected, next argument is: {}", args[i + 1]);
    //         } else {
    //             println!("File flag detected but no argument following it. Exiting.");
    //             exit(1);
    //         }
    //     }
    // }
    check_args();
    let filename = "test.txt";

    // check_args();
    // let mut parser: Option<parser::Parser> = None;

    // match parser::Parser::parse_file(check_args()) {
    //     Ok(p) => {
    //         parser = Some(p);
    //     }
    //     Err(error) => {
    //         println!("Failed to parse file: {:?}", error);
    //     }
    // }

    // if let Some(mut parser) = parser {
    //     parser.map[0][0] = 10;
    //     println!("Parser.map[0][0]: {}", parser.map[0][0]);
    // }
}