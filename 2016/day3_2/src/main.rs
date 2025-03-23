use std::fs::read_to_string;

use day3_2::{parse_all_triangles, valid_sides};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.lines().collect::<Vec<_>>();
    let count = parse_all_triangles(&input)
        .unwrap()
        .iter()
        .map(valid_sides)
        .filter(|&x| x)
        .count();
    println!("{count} valid triangles.")
}
