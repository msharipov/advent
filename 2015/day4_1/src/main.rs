use day4_1::find_lowest_suffix;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim();
    println!("{}", find_lowest_suffix(input));
}
