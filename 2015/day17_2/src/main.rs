use day17_2::*;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let containers = parse_containers(&input).unwrap();
    println!(
        "There are {} ways to get 150 liters of eggnog.",
        count_combinations(&containers, 150)
    );
}
