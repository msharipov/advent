use std::fs::read_to_string;

use day12_2::{Computer, Register, parse_instructions};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let instructions = parse_instructions(&input).unwrap();
    let mut comp = Computer::new(&instructions);
    comp.set_reg(Register::C, 1);
    comp.run();
    println!("a = {}", comp.read_reg(Register::A));
}
