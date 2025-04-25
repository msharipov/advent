use std::fs::read_to_string;

use day17_2::longest_path;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>()[0];
    println!("The length of the longest path is: {}", longest_path(input));
}
