use std::fs::read_to_string;

use day9_2::decompressed_size;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>()[0];
    println!(
        "Expanded text is {} characters long.",
        decompressed_size(input)
    );
}
