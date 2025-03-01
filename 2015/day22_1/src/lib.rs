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
    mana: u64,
    temp_armor: u64,
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
            mana: 500,
            temp_armor: 0,
            effects: vec![],
        }
    }

    pub fn take_damage(&mut self, raw: u64) {
        let mut damage = raw;
        if damage > self.temp_armor {
            damage -= self.temp_armor;
        } else {
            damage = 1;
        }
        if self.health > damage {
            self.health -= damage;
        } else {
            self.health = 0;
        }
    }

    pub fn apply_shield(&mut self) -> Result<(), ()> {
        let shielded = self
            .effects
            .iter()
            .any(|e| matches!(e, Effect::ShieldEffect(_)));
        if !shielded {
            self.effects.push(Effect::ShieldEffect(7));
            return Ok(());
        }
        Err(())
    }

    pub fn update_effects(&mut self) {
        let mut new_effects = vec![];
        for effect in &mut self.effects {
            use Effect::*;
            match effect {
                ShieldEffect(dur) => {
                    self.temp_armor = 7;
                    if *dur > 0 {
                        new_effects.push(ShieldEffect(*dur - 1));
                    }
                }
                _ => (),
            }
        }
    }

    pub fn alive(&self) -> bool {
        self.health > 0
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

    #[test]
    fn player_damage_test_1() {
        let mut player = Player::new(15);
        assert!(player.alive());
        player.apply_shield().unwrap();
        player.update_effects();
        player.take_damage(21);
        assert!(player.alive());
        player.take_damage(10);
        assert!(!player.alive());
    }
}
