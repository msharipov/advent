use std::fs::read_to_string;

use day7_1::supports_ssl;

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let count = input.iter().filter(|line| supports_ssl(line).unwrap()).count();
    println!("{count} IPs support SSL.")
}

