use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").expect("no input file");
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => {},
        }
    }
    println!("floor = {floor}");
}
