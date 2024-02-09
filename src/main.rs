mod idastar;
mod map;
use map::Map;
mod parser;
mod check_args;
use check_args::check_args;

fn main() {
    let map = Map {
        number: 5,
        matrix: vec![
            vec![0, 5, 8],
            vec![4, 1, 7],
            vec![3, 2, 6],
        ],
    };
    check_args();

    idastar::idastar(map);

}