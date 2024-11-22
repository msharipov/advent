use std::fs::read_to_string;

use day2_1::Sides;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let total: u32 = input
        .lines()
        .map(|l| l.parse::<Sides>().unwrap().needed_paper())
        .sum();
    println!("{total} sq. feet of wrapping paper required");
}
