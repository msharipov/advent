use std::fs::read_to_string;

use day15_2::{Disc, first_capsule_time};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let mut discs: Vec<_> = input.iter().map(|s| s.parse::<Disc>().unwrap()).collect();
    discs.push(Disc::new(11, 0));
    let time = first_capsule_time(&discs).unwrap();
    println!("The earliest time to press the button is at t={time}");
}
