use day9_1::GPS;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().split('\n').collect();
    let gps = GPS::new(&input).unwrap();
    println!("shortest distance = {}", gps.shortest_tour().unwrap());
}
