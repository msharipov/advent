use std::fs::read_to_string;

use day7_2::Circuit;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input: Vec<_> = input.trim().split('\n').collect();
    let mut cir = Circuit::new(&input).unwrap();
    let initial = cir.eval("a").unwrap();
    cir.add_wire(&format!("{initial} -> b")).unwrap();
    println!("a = {}", cir.eval("a").unwrap());
}
