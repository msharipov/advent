use std::fs::read_to_string;

use day23_2::{Computer, Instruction, Register};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let instructions = input
        .iter()
        .map(|&line| Instruction::parse(line).unwrap())
        .collect::<Vec<_>>();
    let mut computer = Computer::new(&instructions);
    computer.set(Register::A, 1);
    while computer.execute_next().is_ok() {}
    println!("Value in b: {}", computer.read(Register::B));
}
