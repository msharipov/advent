use std::fs::read_to_string;

use day10_2::{get_outputs, parse_instructions};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let (initial, transfers) = parse_instructions(&input).unwrap();
    let outputs = get_outputs(&initial, &transfers, &[0, 1, 2]);
    let product = outputs.iter().product::<u32>();
    println!("The product is: {product}");
}
