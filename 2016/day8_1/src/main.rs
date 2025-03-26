use std::fs::read_to_string;

use day8_1::{Instruction, Screen};

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
    let total = screen.total_on();
    println!("{total} pixels are on.");
}
