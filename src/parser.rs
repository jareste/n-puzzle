//use std::process::exit;//remove only debug

pub struct Parser {
    pub map: Vec<Vec<i16>>,
    pub size: usize,
}

#[derive(Debug)]
pub enum ParserError {//hacer un enum que devulva parser o error
    FileNotReadable,
    SizeTooLarge,
    InvalidFormat,
}

pub fn parse_file(contents: String) -> Result<Vec<Vec<i16>>, ParserError> {

    let mut lines = contents.lines();

    let mut first_line = match lines.next() {
        Some(line) => line,
        None => {
            println!("Error: File is empty.");
            return Err(ParserError::FileNotReadable)},
    };

    if first_line == "# This puzzle is solvable" || first_line == "# This puzzle is unsolvable" {
        println!("{}", first_line);
        first_line = match lines.next() {
            Some(line) => line,
            None => {
                println!("Error: File is empty.");
                return Err(ParserError::FileNotReadable)},
        };
    }

    

    let size: usize = match first_line.trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(ParserError::FileNotReadable),
    };

    // println!("size: {}", size);
    if size > 25 {
        return Err(ParserError::SizeTooLarge);
    }

    let mut parser = Parser {
        map: vec![vec![0i16; size]; size],
        size: size,
    };
    
    let mut row = 0;
    for line in lines {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() != size {
            return Err(ParserError::InvalidFormat);
        }
        
        let mut col = 0;
        for num_str in numbers {
            match num_str.parse::<i16>() {
                Ok(num) => {
                    parser.map[row][col] = num;
                    col += 1;
                }
                Err(_) => return Err(ParserError::InvalidFormat),
            }
        }
        row += 1;
    }
    Ok(parser.map)
}
