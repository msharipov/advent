use std::fs::read_to_string;

use day8_2::{Instruction, Screen};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let mut screen = Screen::default();
    let instructions = input
        .iter()
        .map(|line| line.parse::<Instruction>().unwrap());
    for inst in instructions {
        screen.apply_instruction(&inst);
    }
    screen.print();
}
