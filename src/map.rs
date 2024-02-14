#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Map {
    pub matrix: Vec<Vec<i16>>,
    pub x: i16,
    pub y: i16,
    pub size: usize,
}


impl Map {
    pub fn find(&self, value: i16) -> (usize, usize) {
        for i in 0..self.size {
            for j in 0..self.size {
                if self.matrix[i][j] == value {
                    return (i, j);
                }
            }
        }
        panic!("Value not found in matrix");
    }

    pub fn manhattan_dist(&self, other: &Map) -> u32 {
        //println!("{:?}", self.matrix);
        let mut dist = 0;
        for i in 0..self.size {
            for j in 0..self.size {
                let value = self.matrix[i][j];
                if value != 0 {
                    let (x, y) = other.find(value);
                    dist += ((i as i32 - x as i32).abs() + (j as i32 - y as i32).abs()) as u32;
                }
            }
        }
        dist
    }

    pub fn update_matrix(&self, dx: i16, dy: i16) -> Vec<Vec<i16>> {
        //println!("x: {} y: {} mat: {:?}",self.x, self.y, self.matrix);
        let mut new_matrix = self.matrix.clone();
        let new_x = (self.x + dx) as usize;
        let new_y = (self.y + dy) as usize;
        new_matrix[new_x][new_y] = 0;
        new_matrix[self.x as usize][self.y as usize] = self.matrix[new_x][new_y];
        //println!("x: {} y: {} mat: {:?}",dx, dy, new_matrix);
        new_matrix
        
    }

    pub fn successors(&self) -> Vec<(Map, u32)> {
        let &Map{matrix: _, x, y, size:_} = self;
        let mut successors = Vec::new();
        if y + 1 < self.size as i16{
            successors.push(Map { matrix: self.update_matrix(0, 1), x, y: y + 1, size: self.size});
        }
        if y - 1 >= 0 {
            successors.push(Map { matrix: self.update_matrix(0, -1), x, y: y - 1, size: self.size });
        }
        if x + 1 < self.size as i16 {
            successors.push(Map { matrix: self.update_matrix(1, 0), x: x + 1, y, size: self.size });
        }
        if x - 1 >= 0 {
            successors.push(Map { matrix: self.update_matrix(-1, 0), x: x - 1, y, size: self.size });
        }

        successors.into_iter().map(|m| (m, 1)).collect()
    }
 }