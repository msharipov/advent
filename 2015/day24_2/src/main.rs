use std::{collections::BTreeSet, fs::read_to_string};

use day24_2::{entanglement, parse_weights, partition_into_quarters};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let weights = BTreeSet::from_iter(parse_weights(&input).unwrap());
    let lowest = parts
        .iter()
        .map(|part| entanglement(part))
        .min()
        .unwrap();
    println!("Lowest entanglement: {lowest}");
}
