use std::{collections::BTreeSet, fs::read_to_string};

use day24_2::{lowest_entanglement, parse_weights};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let weights = BTreeSet::from_iter(parse_weights(&input).unwrap());
    let lowest = lowest_entanglement(weights).unwrap();
    println!("Lowest entanglement: {lowest}");
}
