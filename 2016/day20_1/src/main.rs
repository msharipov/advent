use std::fs::read_to_string;

use day20_1::{lowest_allowed_ip, parse_ranges};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let ranges = parse_ranges(&input).unwrap();
    println!(
        "The lowest allowed IP is {}",
        lowest_allowed_ip(&ranges).unwrap()
    );
}

