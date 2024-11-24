use day8_2::count_repr_chars;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().split('\n').collect();
    let code_count = input.iter().map(|l| l.len()).sum::<usize>();
    let repr_char_count = input.iter().map(|l| count_repr_chars(l)).sum::<usize>();
    println!("difference = {}", repr_char_count - code_count);
}
