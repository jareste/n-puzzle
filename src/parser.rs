use std::fs;
pub struct Parser {
    pub map: Vec<Vec<i16>>,
    pub size: usize,
}

#[derive(Debug)]
pub enum ParserError {//hacer un enum que devulva parser o error
    FileNotFound,
    FileNotReadable,
    SizeTooLarge,
    InvalidFormat,
}
impl Parser {
    pub fn parse_file(file: &str) -> Result<Parser, ParserError> {
        let contents = match fs::read_to_string(file) {
            Ok(contents) => contents,
            Err(_) => return Err(ParserError::FileNotFound),
        };

        let mut lines = contents.lines();
        let first_line = match lines.next() {
            Some(line) => line,
            None => return Err(ParserError::FileNotReadable),
        };

        let size: usize = match first_line.trim().parse() {
            Ok(num) => num,
            Err(_) => return Err(ParserError::FileNotReadable),
        };
        println!("size: {}", size);
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
        Ok(parser)
    }
}