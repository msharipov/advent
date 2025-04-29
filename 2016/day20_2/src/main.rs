use std::fs::read_to_string;

use day20_2::{count_allowed_ips, parse_ranges};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let ranges = parse_ranges(&input).unwrap();
    println!("{} IPs are allowed", count_allowed_ips(&ranges));
}
