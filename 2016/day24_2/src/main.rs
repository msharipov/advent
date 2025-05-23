use std::fs::read_to_string;

use day24_2::Map;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let map = Map::parse_map(&input).unwrap();
    let shortest = map.shortest_trip_length(0).unwrap();
    println!("Shortest trip length is: {shortest}");
}
