use std::fs::read_to_string;

use day4_2::validate_passphrase;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().lines().collect();
    let valid_count = input
        .iter()
        .filter(|phrase| validate_passphrase(phrase))
        .count();
    println!("{valid_count} valid pass phrases");
}
