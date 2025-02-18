use std::fs::read_to_string;

use day21_2::{Armor, Boss, Ring, Shop, Weapon};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let boss = Boss::parse(&input).unwrap();
    let weapons = vec![
        Weapon::new(8, 4),
        Weapon::new(10, 5),
        Weapon::new(25, 6),
        Weapon::new(40, 7),
        Weapon::new(74, 8),
    ];
    let armors = vec![
        Armor::new(13, 1),
        Armor::new(31, 2),
        Armor::new(53, 3),
        Armor::new(75, 4),
        Armor::new(102, 5),
    ];
    let rings = vec![
        Ring::new(25, 1, 0),
        Ring::new(50, 2, 0),
        Ring::new(100, 3, 0),
        Ring::new(20, 0, 1),
        Ring::new(40, 0, 2),
        Ring::new(80, 0, 3),
    ];
    let shop = Shop::new(weapons, armors, rings);
    let lowest_cost = shop.lowest_cost_to_beat(100, &boss).unwrap();
    println!("Lowest cost: {lowest_cost}");
}
