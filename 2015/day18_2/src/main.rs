use std::fs::read_to_string;

use day18_2::{count_on, next_step, parse_matrix};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let mut matrix = parse_matrix(&input).unwrap();
    for _ in 0..100 {
        matrix = next_step(&matrix);
    }
    println!("After 100 steps {} lights are on.", count_on(&matrix));
}
