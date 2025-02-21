pub struct Player {
    health: u64,
    alive: bool,
    mana: u64,
}

impl Player {
    pub fn new(health: u64) -> Player {
        Player {
            health,
            alive: health > 0,
            mana: 500,
        }
    }
}
