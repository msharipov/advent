use std::fs::read_to_string;

use day2_2::get_code;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let code = get_code(&input).unwrap();
    println!("The code is {code}");
}
