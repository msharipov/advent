use std::fs::read_to_string;

use day13_2::Maze;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().parse::<i64>().unwrap();
    let mut maze = Maze::new(input);
    let steps = maze.shortest_distance(&(1, 1), &(31, 39)).unwrap();
    println!("At least {steps} steps are required");
}
