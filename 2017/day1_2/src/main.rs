use std::fs::read_to_string;

use day1_2::captcha_solution;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim();
    let sum = captcha_solution(input).unwrap();
    println!("{sum}");
}
