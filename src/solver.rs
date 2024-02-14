
use core::panic;

use crate::map::Map;
use crate::idastar::ida_star;
use crate::astar::a_star;

fn fill_goal_state(matrix: &mut Vec<Vec<i16>>, n: i16, x: i16, y: i16, dx: i16, dy:i16,  num: i16) {
    if x < 0 || y < 0 || x >= n || y >= n || matrix[x as usize][y as usize] != -1 {
        return;
    }
    matrix[x as usize][y as usize] = num;
    if num == n * n {
        matrix[x as usize][y as usize] = 0;
        return;
    }
    if dx == 0 && dy == 1{
        fill_goal_state(matrix, n, x, y + 1, dx, dy, num + 1);
        fill_goal_state(matrix, n, x + 1, y, 1, 0, num + 1);
    }
    if dx == 1 && dy == 0{
        fill_goal_state(matrix, n, x + 1, y, dx, dy, num + 1);
        fill_goal_state(matrix, n, x, y - 1, 0, -1, num + 1);
    }
    if dx == 0 && dy == -1{
        fill_goal_state(matrix, n, x, y - 1, dx, dy, num + 1);
        fill_goal_state(matrix, n, x - 1, y, -1, 0, num + 1);
    }
    if dx == -1 && dy == 0{
        fill_goal_state(matrix, n, x - 1, y, dx, dy, num + 1);
        fill_goal_state(matrix, n, x, y + 1, 0, 1, num + 1);
    }
}

pub fn get_xy(matrix: & Vec<Vec<i16>>, n: i16) -> (i16, i16) {
    for i in 0..n {
        for j in 0..n {
            if matrix[i as usize][j as usize] == 0 {
                return (i, j);
            }
        }
    }
    panic!("Value not found in matrix");
}

fn get_goal_state(n: i16) -> Map{
    let mut matrix = Vec::new();
    for _i in 0..n {
        let mut _row = vec![-1 as i16; n as usize];
        matrix.push(_row);
    }

    fill_goal_state(&mut matrix, n, 0, 0, 0, 1, 1);

    let (x, y) = get_xy(&mut matrix, n);

    let m : Map = Map {
        matrix,
        x: x,
        y: y,
        size: n as usize,
    };
    m
}


pub fn solver(heuristic: &str,algorithm: &str, h_method: &str,start: &Map, goal: &Map){
    let goal = get_goal_state(3);
    println!("{:?}", goal);
    let map = Map {
        matrix: vec![
            vec![3, 2, 6],
            vec![1, 4, 0],
            vec![8, 7, 5],
        ],
        x: 1,
        y: 2,
        size: 3,
    };
    let result = match algorithm {
        "ida_star" => ida_star(&map, &goal),
        "a_star" => a_star(&map, &goal),
        _ => panic!("Invalid algorithm"),
    };
    println!("Result: {:?}", result);

}
