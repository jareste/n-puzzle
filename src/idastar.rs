use crate::Map;

struct Node {
    pub x: i8,
    pub y: i8,
}

struct Path {
    pub nodes: Vec<Node>,
    pub cost: i16,
}

impl Path {
    pub fn new() -> Path {
        &Path {
            nodes: Vec::new(),
            cost: 0,
        }
    }
}



pub fn idastar(map: Map) {
    println!("idastar called with number: {}", map.number);
    println!("idastar called with matrix: {:?}", map.matrix);

}