use std::fs::read_to_string;

use day13_2::Maze;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().parse::<i64>().unwrap();
    let mut maze = Maze::new(input);
    let cells = maze.accessible_cells(&(1, 1), 50);
    println!("{cells} cells are accessible");
}
