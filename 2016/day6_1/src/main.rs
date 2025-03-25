use std::fs::read_to_string;

use day6_1::decode_message;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let message = decode_message(&input);
    println!("The message is: {message}");
}
