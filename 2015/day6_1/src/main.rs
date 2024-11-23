use day6_1::Grid;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().split('\n').collect();
    let grid = Grid::from_commands(&input).unwrap();
    println!("{} lights on", grid.count_values(true));
}
