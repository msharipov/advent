use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let mut input = input.trim().lines();
}
