use sscanf::sscanf;

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

#[derive(Debug, PartialEq)]
pub struct Boss {
    health: u64,
    damage: u64,
}

impl Boss {
    pub fn parse(lines: &[&str]) -> Result<Self, sscanf::Error> {
        let health = sscanf!(lines[0], "Hit Points: {u64}")?;
        let damage = sscanf!(lines[1], "Damage: {u64}")?;
        Ok(Boss { health, damage })
    }
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

    pub fn take_damage(&mut self, raw: u64) {
        let mut damage = raw;
        let shielded = self
            .effects
            .iter()
            .any(|e| matches!(e, Effect::ShieldEffect(_)));
        if shielded {
            if damage > 7 {
                damage -= 7;
            } else if damage <= 7 {
                damage = 1;
            }
        }
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
            self.alive = false;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boss_parse_test_1() {
        let lines = &["Hit Points: 44", "Damage: 12"];
        let correct = Boss {
            health: 44,
            damage: 12,
        };
        assert_eq!(Boss::parse(lines).unwrap(), correct);
    }
}
