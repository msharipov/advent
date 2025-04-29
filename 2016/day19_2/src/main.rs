use std::fs::read_to_string;

use day19_2::last_elf;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>()[0];
    let count = input.parse::<usize>().unwrap();
    println!(
        "The last elf is #{}",
        last_elf(count.try_into().unwrap()) + 1
    );
}
