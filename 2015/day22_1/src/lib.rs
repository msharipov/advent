pub enum Effect {
    Shield(u64),
    Poison(u64),
    Recharge(u64),
}

pub struct Player {
    health: u64,
    alive: bool,
    mana: u64,
    effects: Vec<Effect>,
}

impl Player {
    pub fn new(health: u64) -> Player {
        Player {
            health,
            alive: health > 0,
            mana: 500,
            effects: vec![],
        }
    }
}
