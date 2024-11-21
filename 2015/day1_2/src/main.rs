use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input file");
    let mut floor = 0;
    for (i, c) in input.chars().enumerate() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {},
        }
        if floor == -1 {
            println!("position = {}", i + 1);
            break;
        }
    }
}
