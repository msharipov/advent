use std::fs::read_to_string;

use day25_1::{compute_code, parse_input};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let (row, col) = parse_input(input[0]).unwrap();
    let code = compute_code(row, col, 20151125);
    println!("The code is: {code}");
}
