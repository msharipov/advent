use sscanf::sscanf;

#[derive(Debug, PartialEq, Clone)]
pub enum Effect {
    ShieldEffect(u64),
    DrainEffect(u64),
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

const SHIELD_COST: u64 = 113;
const SHIELD_DURATION: u64 = 6;
const SHIELD_ARMOR: u64 = 7;

pub struct Player {
    health: u64,
    mana: u64,
    temp_armor: u64,
    effects: Vec<Effect>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Boss {
    health: u64,
    damage: u64,
    effects: Vec<Effect>,
}

impl Boss {
    pub fn parse(lines: &[&str]) -> Result<Self, sscanf::Error> {
        let health = sscanf!(lines[0], "Hit Points: {u64}")?;
        let damage = sscanf!(lines[1], "Damage: {u64}")?;
        Ok(Boss {
            health,
            damage,
            effects: vec![],
        })
    }

    pub fn update_effects(&mut self) {
        let new_effects = vec![];
        for effect in &mut self.effects {
            match effect {
                _ => todo!(),
            }
        }
        self.effects = new_effects;
    }

    pub fn alive(&self) -> bool {
        self.health > 0
    }
}

impl Clone for Player {
    fn clone(&self) -> Self {
        Player {
            effects: self.effects[..].to_vec(),
            ..*self
        }
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
            self.effects.push(Effect::ShieldEffect(SHIELD_DURATION));
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
                    self.temp_armor = SHIELD_ARMOR;
                    if *dur > 0 {
                        new_effects.push(ShieldEffect(*dur - 1));
                    }
                }
                _ => (),
            }
        }
        self.effects = new_effects;
    }

    pub fn reset_effects(&mut self) {
        self.temp_armor = 0;
    }

    pub fn alive(&self) -> bool {
        self.health > 0
    }
}

#[derive(Clone)]
pub struct GameState {
    player: Player,
    boss: Boss,
}

impl GameState {
    pub fn lowest_mana_to_win(max_depth: u64) -> Option<u64> {
        None
    }
}

fn recursive_step(
    mut state: GameState,
    cur_depth: u64,
    max_depth: u64,
    action: Spell,
    mut spent_mana: u64,
) -> Option<u64> {
    if cur_depth >= max_depth {
        return None;
    }
    state.player.update_effects();
    state.boss.update_effects();
    match action {
        Spell::Shield => {
            if state.player.mana < SHIELD_COST {
                return None;
            }
            state.player.mana -= SHIELD_COST;
            spent_mana += SHIELD_COST;
            if state.player.apply_shield().is_err() {
                return None;
            }
        }
        _ => todo!(),
    }
    state.player.reset_effects();
    state.player.update_effects();
    state.boss.update_effects();
    if !state.boss.alive() {
        return Some(spent_mana);
    }
    state.player.take_damage(state.boss.damage);
    state.player.reset_effects();
    if !state.player.alive() {
        return None;
    }

    // Explore the next possible moves
    let mut lowest_mana = None;
    let shield_next = recursive_step(
        state.clone(),
        cur_depth + 1,
        max_depth,
        Spell::Shield,
        spent_mana,
    );
    if shield_next.is_some() {
        match lowest_mana {
            None => {
                lowest_mana = shield_next;
            }
            Some(mana) => {
                if mana > shield_next.unwrap() {
                    lowest_mana = shield_next;
                }
            }
        }
    }
    lowest_mana
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
            effects: vec![],
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
