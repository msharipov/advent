use std::fs::read_to_string;

use day22_2::{Node, count_viable_pairs};
use itertools::Itertools;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let nodes = input
        .iter()
        .skip(2)
        .map(|line| line.parse::<Node>().unwrap())
        .collect_vec();
    println!("{} viable pairs", count_viable_pairs(&nodes));
}
