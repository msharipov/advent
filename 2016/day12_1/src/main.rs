use std::fs::read_to_string;

use day12_1::{Computer, parse_instructions};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let instructions = parse_instructions(&input).unwrap();
    let mut comp = Computer::new(&instructions);
    comp.run();
    println!("a = {}", comp.read_reg(day12_1::Register::A));
}
