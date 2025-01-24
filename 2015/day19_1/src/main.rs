use std::fs::read_to_string;

use day19_1::{count_new_sequences, parse_replacements, parse_sequence};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let mut input = input.trim().lines();
    let repl = parse_replacements(&mut input).unwrap();
    let seq = parse_sequence(&mut input).unwrap();
    let count = count_new_sequences(&seq, &repl);
    println!("{count} possible sequences");
}

