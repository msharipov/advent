use std::fs::read_to_string;

use day10_1::generate_next;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let mut input = input.trim().to_string();
    for _ in 0..40 {
        input = generate_next(&input);
    }
    println!("length = {}", input.len());
}
