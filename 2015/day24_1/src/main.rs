use std::{collections::BTreeSet, fs::read_to_string};

use day24_1::{entanglement, parse_weights, partition_into_thirds};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let weights = BTreeSet::from_iter(parse_weights(&input).unwrap());
    let parts = partition_into_thirds(weights).unwrap();
    let lowest = parts
        .iter()
        .map(entanglement)
        .min()
        .unwrap();
    println!("Lowest entanglement: {lowest}");
}
