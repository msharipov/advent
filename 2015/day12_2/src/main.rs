use day12_2::count_numbers;
use serde_json::{from_str, Value};
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let json: Value = from_str(input.trim()).unwrap();
    println!("Total sum: {}", count_numbers(&json).unwrap());
}
