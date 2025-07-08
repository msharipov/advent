use std::fs::read_to_string;

use day8_1::{Instruction, State};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().lines().collect();
    let instructions: Vec<_> = input
        .iter()
        .map(|line| line.parse::<Instruction>().unwrap())
        .collect();
    let mut state = State::default();
    for inst in &instructions {
        state.apply(inst);
    }
    let highest = state.read(&state.max_register().unwrap()).unwrap();
    println!("Highest value: {highest}");
}
