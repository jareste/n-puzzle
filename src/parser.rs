use crate::map::Map;
use std::process::exit;


#[derive(Debug)]
pub enum ParserError {
    FileNotReadable,
    SizeTooLarge,
    InvalidFormat,
}

fn find_zero(matrix: &Vec<Vec<usize>>) -> (isize, isize) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                return (i as isize, j as isize);
            }
        }
    }
    println!("Value not found in matrix. Exiting.");
    exit(1);
}
pub fn parse_file(contents: String) -> Result<Map, ParserError> {
    let mut lines = contents.lines();

    let mut first_line = match lines.next() {
        Some(line) => line,
        None => {
            println!("Error: File is empty.");
            return Err(ParserError::FileNotReadable)},
    };

    if first_line.starts_with("#") {
        first_line = match lines.next() {
            Some(line) => line,
            None => {
                println!("Error: File is empty.");
                return Err(ParserError::FileNotReadable)},
        };
    }

    let size: usize = match first_line.split('#').next().unwrap().trim().parse() {
        Ok(num) => num,
        Err(_) => return Err(ParserError::FileNotReadable),
    };

    if size > 25 {
        return Err(ParserError::SizeTooLarge);
    }

    let mut map = Map {
        matrix: vec![vec![0usize; size]; size],
        x: 0,
        y: 0,
        size: size,
    };
    
    let mut row = 0;
    for line in lines {
        let line = line.split('#').next().unwrap(); // Ignore everything after #
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() != size {
            return Err(ParserError::InvalidFormat);
        }
        
        let mut col = 0;
        for num_str in numbers {
            match num_str.parse::<usize>() {
                Ok(num) => {
                    if num > size * size {
                        println!("Number {} too large for matrix size.", num);
                        exit(1);
                    }
                    map.matrix[row][col] = num;
                    col += 1;
                }
                Err(_) => return Err(ParserError::InvalidFormat),
            }
        }
        row += 1;
    }
    (map.x, map.y) = find_zero(&map.matrix);
    Ok(map)
}