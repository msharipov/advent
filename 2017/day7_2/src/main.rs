use std::fs::read_to_string;

use day7_1::{ParsedNode, bottom_node};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().lines().collect();
    let nodes: Vec<_> = input
        .iter()
        .map(|line| line.parse::<ParsedNode>().unwrap())
        .collect();
    let bottom = bottom_node(&nodes).unwrap();
    println!("{}", bottom.get_name());
}
