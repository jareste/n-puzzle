use std::process::exit;//remove only debug
use crate::map::Map;

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

fn find_zero(matrix: &Vec<Vec<usize>>) -> (isize, isize) {
    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            if matrix[i][j] == 0 {
                return (i as isize, j as isize);
            }
        }
    }
    panic!("Value not found in matrix");
}

pub fn parse_file(contents: String) -> Result<Map, ParserError> {

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

    let mut map = Map {
        matrix: vec![vec![0usize; size]; size],
        x: 0,
        y: 0,
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
            match num_str.parse::<usize>() {
                Ok(num) => {
                    map.matrix[row][col] = num;
                    col += 1;
                }
                Err(_) => return Err(ParserError::InvalidFormat),
            }
        }
        row += 1;
    }
    (map.x, map.y) = find_zero(&map.matrix);
    println!("map.x: {} map.y: {}", map.x, map.y);
    // println!("map.matrix: {}", map.matrix[map.x as usize][map.y as usize]);
    Ok(map)
}
