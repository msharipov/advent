use day5_1::is_nice;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let count = input.lines().filter(|s| is_nice(s)).count();
    println!("{count} nice strings");
}
