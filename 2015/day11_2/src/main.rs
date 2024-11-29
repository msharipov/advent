use std::fs::read_to_string;

use day11_2::next_password;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().to_string();
    println!("next password: {}", next_password(&next_password(&input)));
}
