mod parser;

fn main() {
    let filename = "test.txt";

    let mut parser: Option<parser::Parser> = None;

    match parser::Parser::parse_file(filename) {
        Ok(p) => {
            parser = Some(p);
        }
        Err(error) => {
            println!("Failed to parse file: {:?}", error);
        }
    }

    if let Some(mut parser) = parser {
        parser.map[0][0] = 10;
        println!("Parser.map[0][0]: {}", parser.map[0][0]);
    }
}