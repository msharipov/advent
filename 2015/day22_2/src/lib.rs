use std::cmp::min;

use sscanf::sscanf;

#[derive(Debug, PartialEq, Clone)]
pub enum Effect {
    ShieldEffect(u64),
    DrainEffect(u64),
    PoisonEffect(u64),
    RechargeEffect(u64),
}

#[derive(Clone, Debug, PartialEq)]
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
const MISSILE_COST: u64 = 53;
const MISSILE_DAMAGE: u64 = 4;
const DRAIN_COST: u64 = 73;
const DRAIN_DAMAGE: u64 = 2;
const POISON_COST: u64 = 173;
const POISON_DURATION: u64 = 6;
const POISON_DAMAGE: u64 = 3;
const RECHARGE_COST: u64 = 229;
const RECHARGE_DURATION: u64 = 5;
const RECHARGE_AMOUNT: u64 = 101;

#[derive(Debug)]
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
    poison: Option<Effect>,
}

impl Boss {
    pub fn new(health: u64, damage: u64) -> Self {
        Boss {
            health,
            damage,
            poison: None,
        }
    }

    pub fn parse(lines: &[&str]) -> Result<Self, sscanf::Error> {
        let health = sscanf!(lines[0], "Hit Points: {u64}")?;
        let damage = sscanf!(lines[1], "Damage: {u64}")?;
        Ok(Boss::new(health, damage))
    }

    pub fn update_effects(&mut self) {
        if let Some(Effect::PoisonEffect(turns)) = &self.poison {
            if *turns > 0 {
                self.health -= min(self.health, POISON_DAMAGE);
            }
            if *turns > 1 {
                self.poison = Some(Effect::PoisonEffect(turns - 1))
            } else {
                self.poison = None;
            }
        }
    }

    pub fn apply_poison(&mut self) -> Result<(), ()> {
        if self.poison.is_some() {
            return Err(());
        }
        self.poison = Some(Effect::PoisonEffect(POISON_DURATION));
        Ok(())
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

    pub fn apply_recharge(&mut self) -> Result<(), ()> {
        let recharging = self
            .effects
            .iter()
            .any(|e| matches!(e, Effect::RechargeEffect(_)));
        if !recharging {
            self.effects.push(Effect::RechargeEffect(RECHARGE_DURATION));
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
                    if *dur > 1 {
                        new_effects.push(ShieldEffect(*dur - 1));
                    }
                }
                RechargeEffect(dur) => {
                    self.mana += RECHARGE_AMOUNT;
                    if *dur > 1 {
                        new_effects.push(RechargeEffect(*dur - 1));
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

#[derive(Debug, Clone)]
pub struct GameState {
    player: Player,
    boss: Boss,
}

impl GameState {
    pub fn new(player: Player, boss: Boss) -> Self {
        GameState { player, boss }
    }

    pub fn lowest_mana_to_win(&self, max_depth: u64) -> Option<(u64, Vec<Spell>)> {
        let mut lowest_mana = None;
        let mut cheapest_spells = vec![];
        let spells = [
            Spell::Shield,
            Spell::MagicMissile,
            Spell::Recharge,
            Spell::Poison,
            Spell::Drain,
        ];
        let mana_spent_results = spells
            .iter()
            .filter_map(|spell| recursive_step(self.clone(), 0, max_depth, spell.clone(), 0, &[]));
        for (mana_spent, used_spells) in mana_spent_results {
            match lowest_mana {
                None => {
                    lowest_mana = Some(mana_spent);
                    cheapest_spells = used_spells;
                }
                Some(mana) => {
                    if mana > mana_spent {
                        lowest_mana = Some(mana_spent);
                        cheapest_spells = used_spells;
                    }
                }
            }
        }
        lowest_mana.map(|mana| (mana, cheapest_spells))
    }
}

fn recursive_step(
    mut state: GameState,
    cur_depth: u64,
    max_depth: u64,
    action: Spell,
    mut spent_mana: u64,
    used_spells: &[Spell],
) -> Option<(u64, Vec<Spell>)> {
    if cur_depth >= max_depth {
        return None;
    }
    state.player.take_damage(1);
    if !state.player.alive() {
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
        Spell::MagicMissile => {
            if state.player.mana < MISSILE_COST {
                return None;
            }
            state.player.mana -= MISSILE_COST;
            spent_mana += MISSILE_COST;
            state.boss.health -= min(state.boss.health, MISSILE_DAMAGE);
        }
        Spell::Drain => {
            if state.player.mana < DRAIN_COST {
                return None;
            }
            state.player.mana -= DRAIN_COST;
            spent_mana += DRAIN_COST;
            state.boss.health -= min(state.boss.health, DRAIN_DAMAGE);
            state.player.health += DRAIN_DAMAGE;
        }
        Spell::Poison => {
            if state.player.mana < POISON_COST {
                return None;
            }
            state.player.mana -= POISON_COST;
            spent_mana += POISON_COST;
            if state.boss.apply_poison().is_err() {
                return None;
            }
        }
        Spell::Recharge => {
            if state.player.mana < RECHARGE_COST {
                return None;
            }
            state.player.mana -= RECHARGE_COST;
            spent_mana += RECHARGE_COST;
            if state.player.apply_recharge().is_err() {
                return None;
            }
        }
    }
    let mut used_spells = used_spells.to_vec();
    used_spells.push(action);
    state.player.reset_effects();
    // End of player's turn

    state.player.update_effects();
    state.boss.update_effects();
    if !state.boss.alive() {
        return Some((spent_mana, used_spells));
    }
    state.player.take_damage(state.boss.damage);
    state.player.reset_effects();
    if !state.player.alive() {
        return None;
    }

    // Explore the next possible moves
    let mut lowest_mana = None;
    let spells = [
        Spell::Shield,
        Spell::MagicMissile,
        Spell::Drain,
        Spell::Poison,
        Spell::Recharge,
    ];
    let mana_spent_results = spells.iter().filter_map(|spell| {
        recursive_step(
            state.clone(),
            cur_depth + 1,
            max_depth,
            spell.clone(),
            spent_mana,
            &used_spells,
        )
    });
    let mut used_spells = vec![];
    for (mana_spent, spells_sequence) in mana_spent_results {
        match lowest_mana {
            None => {
                lowest_mana = Some(mana_spent);
                used_spells = spells_sequence;
            }
            Some(mana) => {
                if mana > mana_spent {
                    lowest_mana = Some(mana_spent);
                    used_spells = spells_sequence;
                }
            }
        }
    }
    lowest_mana.map(|mana| (mana, used_spells))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn boss_parse_test_1() {
        let lines = &["Hit Points: 44", "Damage: 12"];
        let correct = Boss::new(44, 12);
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

    #[test]
    fn lowest_mana_to_win_test_1() {
        let player = Player::new(15);
        let boss = Boss::new(15, 1);
        let state = GameState { player, boss };
        assert_eq!(state.lowest_mana_to_win(6).unwrap().0, 53 * 4);
    }

    #[test]
    fn lowest_mana_to_win_test_2() {
        let player = Player::new(15);
        let boss = Boss::new(14, 1);
        let state = GameState { player, boss };
        assert!(state.lowest_mana_to_win(2).is_none());
    }

    #[test]
    fn lowest_mana_to_win_test_3() {
        let player = Player::new(15);
        let boss = Boss::new(15, 6);
        let state = GameState { player, boss };
        assert_eq!(
            state.lowest_mana_to_win(6).unwrap().0,
            MISSILE_COST * 2 + POISON_COST
        );
    }
}
