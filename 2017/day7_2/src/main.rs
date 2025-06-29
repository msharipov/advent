use std::fs::read_to_string;

use day7_2::{Node, ParsedNode};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().lines().collect();
    let nodes: Vec<_> = input
        .iter()
        .map(|line| line.parse::<ParsedNode>().unwrap())
        .collect();
    let node_tree = Node::new(&nodes).unwrap();
    let weight = node_tree.incorrect_node_target_weight().unwrap();
    println!("The weight should be: {weight}");
}
