use std::fs::read_to_string;

use day2_2::Sides;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let total: u32 = input
        .lines()
        .map(|l| l.parse::<Sides>().unwrap().needed_ribbon())
        .sum();
    println!("{total} feet of ribbon required");
}
