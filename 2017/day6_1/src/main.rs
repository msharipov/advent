use std::fs::read_to_string;

use day6_1::Memory;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().lines().collect();
    let mem = input[0].parse::<Memory>().unwrap();
    println!("{} redistribution cycles", mem.cycle_period());
}
