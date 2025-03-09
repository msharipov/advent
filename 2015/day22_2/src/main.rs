use std::fs::read_to_string;

use day22_2::{Boss, GameState, Player};

fn main() {
    let input = read_to_string("input.txt").expect("no input.txt in current directory");
    let input = input.trim().lines().collect::<Vec<_>>();
    let player = Player::new(50);
    let boss = Boss::parse(&input).unwrap();
    let state = GameState::new(player, boss);
    match state.lowest_mana_to_win(10) {
        None => {
            println!("No solution found!");
        }
        Some((mana, spells)) => {
            println!("Need {mana} mana to win. Spells used:");
            for spell in spells {
                println!("{spell:?}");
            }
        }
    }
}
