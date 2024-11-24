use std::fs::read_to_string;

use day7_1::Circuit;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().split('\n').collect();
    let cir = Circuit::new(&input).unwrap();
    println!("a = {}", cir.eval("a").unwrap());
}
