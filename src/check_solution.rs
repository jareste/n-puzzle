// first i should index the matrix i got from the input
struct Map {
    pub matrix: Vec<Vec<usize>>,
    pub x: isize,
    pub y: isize,
    pub size: usize,
}

fn generate_goal(size: usize) -> Map {
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
            map.matrix[x as usize][y as usize] = counter;
            map.x = x;
            map.y = y;
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

    // println!("Goal:");
    // for i in 0..size {
    //     println!("{:?}", map.matrix[i]);
    // }
    map
}
// de aqui arriba va fuera de este archivo


pub fn check_solution(matrix: &Vec<Vec<i16>>) -> bool {

    let mut line_matrix: Vec<i16> = vec![-1; matrix[0].len() * matrix[0].len()];

    for i in 0..matrix[0].len() {
        for j in 0..matrix[0].len() {
            if matrix[i][j] == 0 {
                line_matrix[matrix[0].len() * i + j] = 0;
            } else {
                line_matrix[matrix[0].len() * i + j] = matrix[i][j];
            }
        }
    }

    let map = generate_goal(matrix[0].len());

    let mut line_goal: Vec<i16> = vec![-1; matrix[0].len() * matrix[0].len()];
    for i in 0..matrix[0].len() {
        for j in 0..matrix[0].len() {
            if map.matrix[i][j] == 0 {
                line_goal[matrix[0].len() * i + j] = 0;
            } else {
                line_goal[matrix[0].len() * i + j] = map.matrix[i][j] as i16;
            }
        }
    }
    println!("before indexing");
    println!("{:?}", line_matrix);

    for i in 0..line_matrix.len() {
        for j in 0..line_goal.len() {
            if line_matrix[i] == line_goal[j] {
                if (j + 1) == line_goal.len() {
                    println!("entro:{}{}", j, line_goal.len());
                    line_matrix[i] = 0;
                    break;
                }
                line_matrix[i] = (j + 1) as i16;
                break;
            }
        }
    }
    println!("after indexing");
    println!("{:?}", line_matrix);
    println!("goal as line:");
    println!("{:?}", line_goal);
    true
}