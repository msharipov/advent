pub enum Effect {
    ShieldEffect(u64),
    PoisonEffect(u64),
    RechargeEffect(u64),
}

pub enum Spell {
    MagicMissile,
    Drain,
    Shield,
    Poison,
    Recharge,
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
