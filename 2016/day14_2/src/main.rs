use std::fs::read_to_string;

use day14_2::index_of_nth_key;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let index = index_of_nth_key(64.try_into().unwrap(), input[0]);
    println!("The index of 64th key is {index}");
}
