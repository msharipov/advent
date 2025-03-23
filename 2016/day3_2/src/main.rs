use std::fs::read_to_string;

use day3_2::{parse_sides, valid_sides};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.lines().collect::<Vec<_>>();
    let count = input
        .iter()
        .map(|line| parse_sides(line).unwrap())
        .map(|sides| valid_sides(&sides))
        .filter(|&x| x)
        .count();
    println!("{count} valid triangles.")
}
