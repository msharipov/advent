use std::fs::read_to_string;

use day17_1::{moves_to_string, shortest_path};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>()[0];
    let path = shortest_path(input).unwrap();
    println!("The shortest path is: {}", moves_to_string(&path));
}
