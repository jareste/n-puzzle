mod idastar;
mod map;
use map::Map;

fn main() {
    let map = Map {
        number: 5,
        matrix: vec![
            vec![0, 5, 8],
            vec![4, 1, 7],
            vec![3, 2, 6],
        ],
    };
    idastar::idastar(map);
}