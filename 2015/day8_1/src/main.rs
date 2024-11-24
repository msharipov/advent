use day8_1::count_chars;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().split('\n').collect();
    let code_count = input.iter().map(|l| l.len()).sum::<usize>();
    let char_count = input
        .iter()
        .map(|l| count_chars(l).unwrap())
        .sum::<usize>();
    println!("difference = {}", code_count - char_count);
}
