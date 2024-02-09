
use crate::map::Map;
use pathfinding::prelude::idastar;

fn fill_goal_state(matrix: &mut Vec<Vec<i16>>, n: i16, x: i16, y: i16, dx: i16, dy:i16,  num: i16) {
    if x < 0 || y < 0 || x >= n || y >= n || matrix[x as usize][y as usize] != -1 {
        return;
    }
    matrix[x as usize][y as usize] = num;
    if num == n * n {
        matrix[x as usize][y as usize] = 0;
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

fn get_goal_state(n: i16) -> Map{
    let mut matrix = Vec::new();
    for _i in 0..n {
        let mut _row = vec![-1 as i16; n as usize];
        matrix.push(_row);
    }

    fill_goal_state(&mut matrix, n, 0, 0, 0, 1, 1);

    let m : Map = Map {
        matrix,
        x: n - 1,
        y: n - 1,
        size: n as usize,
    };
    m
}


pub fn my_idastar() {
    let m = get_goal_state(3);
    println!("{:?}", m);
    let goal: Map = Map {
        matrix: vec![
            vec![1, 2, 3, 4, 5],
            vec![16, 17, 18, 19, 6],
            vec![15, 24, 0, 20, 7],
            vec![14, 23, 22, 21, 8],
            vec![13, 12, 11, 10, 9],
        ],
        x: 2,
        y: 2,
        size: 5,
    };
    let map = Map {
        matrix: vec![
            vec![15, 6, 18, 24, 8],
            vec![9, 0, 12, 17, 16],
            vec![13, 5, 3, 22, 7],
            vec![2, 21, 11, 4, 19],
            vec![23, 14, 10, 20, 1],
        ],
        x: 1,
        y: 1,
        size: 5,
    };
    println!("Map3.2: {}", map.matrix[1][1]);
    let result = idastar(&map, |m| m.successors(), |m| m.manhattan_dist(&goal),
                        |m| *m == goal);
    println!("Result: {:?}", result);
 }
