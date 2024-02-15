use crate::map::Map;

pub fn generate_goal(size: usize) -> Map {
    let mut map = Map {
        matrix: vec![vec![0; size]; size],
        x: 0,
        y: 0,
        size: size,
    };
    let mut counter = 0;

    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut dx: isize = 0;
    let mut dy: isize = 1;

    while counter < size * size {
        if x >= 0 && x < size as isize && y >= 0 && y < size as isize {
            counter += 1;
            map.matrix[x as usize][y as usize] = counter as usize;
            map.x = x as isize;
            map.y = y as isize;
        }

        if x + dx >= size as isize || x + dx < 0 || y + dy >= size as isize || y + dy < 0 || map.matrix[(x + dx) as usize][(y + dy) as usize] != 0 {
            let temp = dx;
            dx = dy;
            dy = -temp;
        }

        x += dx;
        y += dy;
    }

    map.matrix[map.x as usize][map.y as usize] = 0;

    println!("Goal:");
    for i in 0..size {
        println!("{:?}", map.matrix[i]);
    }
    map
}
