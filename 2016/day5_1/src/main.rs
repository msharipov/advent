use std::fs::read_to_string;

use day5_1::compute_code;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let code = compute_code(input[0]);
    println!("The door code is: {code}");
}
