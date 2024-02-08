struct Map(i16, i16);

struct Parser {
    map: Map,
    size: char,
}
fn main() {
    // a[5] = 0;

    let mut parser = Parser {
        map: Map(5, 5),
        size: 5 as char,
    };

    parser.map.0 = 10;

    println!("Parser.map.0: {}", parser.map.0);


}