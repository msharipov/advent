use std::fs::read_to_string;
use day16_2::{Dogs, Sue};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines();
    let mut sues = input.map(|line| Sue::new(line).unwrap());
    let clues = Sue {
        children: Some(3),
        cats: Some(7),
        dogs: Dogs {
            samoyeds: Some(2),
            pomeranians: Some(3),
            akitas: Some(0),
            vizslas: Some(0),
        },
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
        ..Default::default()
    };
    let matching_sue = sues.find(|sue| sue.compatible(&clues));
    println!("The number of the right aunt Sue is {}.", matching_sue.unwrap().number);
}
