
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Map {
    pub matrix: Vec<Vec<usize>>,
    pub x: isize,
    pub y: isize,
    pub size: usize,
}

static mut GOAL_VEC: Vec<(usize, usize)> = Vec::new();


impl Map {
    pub fn find(&self, goal: &Map, value: usize) -> (usize, usize) {
        if false{
            for i in 0..self.size {
                for j in 0..self.size {
                    if self.matrix[i][j] == value  as usize{
                        return (i, j);
                    }
                }
            }
            panic!("Value not found in matrix");
        }
        else{
            unsafe{
                if GOAL_VEC.is_empty(){
                    GOAL_VEC = vec![(0, 0); (self.size * self.size) as usize];
                    for i in 0..self.size {
                        for j in 0..self.size {
                            let aux = goal.matrix[i][j];
                            GOAL_VEC[aux as usize] = (i, j);
                            // println!("{:?} ", GOAL_VEC[aux as usize])
                        }
                    }
                }
                return GOAL_VEC[value as usize];
            }
        }
    }

    
    pub fn manhattan_dist(&self, other: &Map) -> u32 {
        //println!("{:?}", self.matrix);
        let mut dist = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                let value = self.matrix[i][j];
                if value != 0 {
                    let (x, y) = other.find(other, value);
                    dist += ((i as i32 - x as i32).abs() + (j as i32 - y as i32).abs()) as u32;
                }
            }
        }
        dist
    }


    pub fn euclidean_dist(&self, other: &Map) -> u32 {
        let mut dist = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                let value = self.matrix[i][j];
                if value != 0 {
                    let (x, y) = other.find(other, value);
                    let dx = (i as i32 - x as i32).abs();
                    let dy = (j as i32 - y as i32).abs();
                    dist += (((dx).pow(2) + (dy).pow(2)) as f64).sqrt() as u32;
                }
            }
        }
        dist
    }

    pub fn hamming_dist(&self, other: &Map) -> u32 {
        let mut dist = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                if self.matrix[i][j] != other.matrix[i][j] {
                    dist += 1;
                }
            }
        }
        dist
    }

    pub fn manhattan_linear_conflicts(&self, other: &Map) -> u32 {
        let mut dist = self.manhattan_dist(other);

        for i in 0..self.size {
            for j in 0..self.size{
                let value = self.matrix[i][j];
                if value != 0 {
                    let (x, y) = other.find(other, value);
                    if i == x {
                        for k in j+1..self.size {
                            let value2 = self.matrix[i][k];
                            if value2 != 0 {
                                let (x2, y2) = other.find(other, value2);
                                if x2 == i && y2 < y {
                                    dist += 2;
                                }
                            }
                        }
                    }
                    if j == y {
                        for k in i+1..self.size {
                            let value2 = self.matrix[k][j];
                            if value2 != 0 {
                                let (x2, y2) = other.find(other, value2);
                                if y2 == j && x2 < x {
                                    dist += 2;
                                }
                            }
                        }
                    }
                }
            }
        }
        dist
    }

    pub fn update_matrix(&self, dx: isize, dy: isize) -> Vec<Vec<usize>> {
        let mut new_matrix = self.matrix.clone();
        let new_x = (self.x + dx) as usize;
        let new_y = (self.y + dy) as usize;
        new_matrix[new_x][new_y] = 0;
        new_matrix[self.x as usize][self.y as usize] = self.matrix[new_x][new_y];
        new_matrix
        
    }

    pub fn successors(&self) -> Vec<(Map, u32)> {
        let &Map{matrix: _, x, y, size:_} = self;
        let mut successors = Vec::new();
        if y + 1 < self.size as isize{
            successors.push(Map { matrix: self.update_matrix(0, 1), x, y: y + 1, size: self.size});
        }
        if y - 1 >= 0 {
            successors.push(Map { matrix: self.update_matrix(0, -1), x, y: y - 1, size: self.size });
        }
        if x + 1 < self.size as isize {
            successors.push(Map { matrix: self.update_matrix(1, 0), x: x + 1, y, size: self.size });
        }
        if x - 1 >= 0 {
            successors.push(Map { matrix: self.update_matrix(-1, 0), x: x - 1, y, size: self.size });
        }

        successors.into_iter().map(|m| (m, 1)).collect()
    }
 }
