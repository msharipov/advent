use std::fs::read_to_string;

use day16_1::{checksum, generate_data};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let data = generate_data(input[0], 272);
    println!("The checksum is {}", checksum(&data));
}

