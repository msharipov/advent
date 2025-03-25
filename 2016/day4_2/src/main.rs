use std::fs::read_to_string;

use day4_2::parse_rooms;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.lines().collect::<Vec<_>>();
    let found = parse_rooms(&input)
        .unwrap()
        .into_iter()
        .filter(|room| room.is_valid())
        .filter(|room| room.decrypt_name().contains("object"));
    for room in found {
        println!("{room:?}");
    }
}
