use itertools::Itertools;

#[derive(Default, Clone)]
pub struct Weapon {
    cost: i64,
    damage: i64,
}

#[derive(Default, Clone)]
pub struct Armor {
    cost: i64,
    armor: i64,
}

#[derive(Default, Clone)]
pub struct Ring {
    cost: i64,
    damage: i64,
    armor: i64,
}

#[derive(Default)]
pub struct Player {
    pub weapon: Weapon,
    pub armor: Option<Armor>,
    pub left_ring: Option<Ring>,
    pub right_ring: Option<Ring>,
    pub health: i64,
}

pub struct Shop {
    weapons: Vec<Weapon>,
    armor: Vec<Armor>,
    rings: Vec<Ring>,
}

impl Shop {
    pub fn lowest_cost_to_beat(&self, player_hp: i64, boss: &Boss) -> Option<i64> {
        let weapons = self.weapons.clone();
        let mut armors = self.armor.iter().map(|a| Some(a.clone())).collect_vec();
        armors.push(None);
        let mut rings = self.rings.iter().map(|r| Some(r.clone())).collect_vec();
        rings.push(None);
        rings.push(None);
        let ring_combos = rings.iter().combinations(2);
        let mut lowest_cost = None;
        for equipment in weapons
            .iter()
            .cartesian_product(armors)
            .cartesian_product(ring_combos)
        {
            let mut player = Player::new(player_hp);
            player.weapon = equipment.0 .0.clone();
            player.armor = equipment.0 .1.clone();
            player.left_ring = equipment.1[0].clone();
            player.right_ring = equipment.1[1].clone();
            if player.beats(boss) {
                let cost = player.equipment_cost();
                if lowest_cost.is_none() || cost < lowest_cost.unwrap() {
                    lowest_cost = Some(cost);
                }
            }
        }
        lowest_cost
    }
}

#[derive(Debug, PartialEq)]
pub struct Boss {
    health: i64,
    damage: i64,
    armor: i64,
}

impl Boss {
    pub fn parse(lines: &[&str]) -> Result<Self, sscanf::Error> {
        let health = sscanf::sscanf!(lines[0], "Hit Points: {i64}")?;
        let damage = sscanf::sscanf!(lines[1], "Damage: {i64}")?;
        let armor = sscanf::sscanf!(lines[2], "Armor: {i64}")?;
        Ok(Boss {
            health,
            damage,
            armor,
        })
    }
}

impl Player {
    pub fn new(hp: i64) -> Self {
        Player {
            health: hp,
            ..Player::default()
        }
    }

    pub fn damage(&self) -> i64 {
        let mut damage = self.weapon.damage;
        if let Some(ring) = &self.left_ring {
            damage += ring.damage;
        }
        if let Some(ring) = &self.right_ring {
            damage += ring.damage;
        }
        damage
    }

    pub fn armor(&self) -> i64 {
        let mut armor = 0;
        if let Some(a) = &self.armor {
            armor += a.armor;
        }
        if let Some(a) = &self.left_ring {
            armor += a.armor;
        }
        if let Some(a) = &self.right_ring {
            armor += a.armor;
        }
        armor
    }

    pub fn equipment_cost(&self) -> i64 {
        let mut cost = self.weapon.cost;
        if let Some(a) = &self.armor {
            cost += a.cost;
        }
        if let Some(r) = &self.left_ring {
            cost += r.cost;
        }
        if let Some(r) = &self.right_ring {
            cost += r.cost;
        }
        cost
    }

    pub fn beats(&self, boss: &Boss) -> bool {
        let mut player_hp = self.health;
        let mut boss_hp = boss.health;
        while player_hp > 0 {
            boss_hp -= self.damage() - boss.armor;
            if boss_hp <= 0 {
                return true;
            }
            player_hp -= boss.damage - self.armor();
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn player_damage_test_1() {
        assert_eq!(Player::default().damage(), 0);
    }

    #[test]
    fn player_damage_test_2() {
        let sword = Weapon { cost: 3, damage: 4 };
        let armor = Armor { cost: 5, armor: 8 };
        let ring1 = Ring {
            cost: 5,
            damage: 2,
            armor: 1,
        };
        let ring2 = Ring {
            cost: 9,
            damage: 4,
            armor: 2,
        };
        let player = Player {
            weapon: sword,
            armor: Some(armor),
            left_ring: Some(ring1),
            right_ring: Some(ring2),
            health: 100,
        };
        assert_eq!(player.damage(), 10);
    }

    #[test]
    fn player_armor_test_1() {
        assert_eq!(Player::default().armor(), 0);
    }

    #[test]
    fn player_armor_test_2() {
        let sword = Weapon { cost: 3, damage: 4 };
        let armor = Armor { cost: 5, armor: 8 };
        let ring1 = Ring {
            cost: 5,
            damage: 2,
            armor: 1,
        };
        let ring2 = Ring {
            cost: 9,
            damage: 4,
            armor: 2,
        };
        let player = Player {
            weapon: sword,
            armor: Some(armor),
            left_ring: Some(ring1),
            right_ring: Some(ring2),
            health: 100,
        };
        assert_eq!(player.armor(), 11);
    }

    #[test]
    fn player_equipment_cost_test_1() {
        assert_eq!(Player::default().equipment_cost(), 0);
    }

    #[test]
    fn player_equipment_cost_test_2() {
        let sword = Weapon { cost: 3, damage: 4 };
        let armor = Armor { cost: 5, armor: 8 };
        let ring1 = Ring {
            cost: 5,
            damage: 2,
            armor: 1,
        };
        let ring2 = Ring {
            cost: 9,
            damage: 4,
            armor: 2,
        };
        let player = Player {
            weapon: sword,
            armor: Some(armor),
            left_ring: Some(ring1),
            right_ring: Some(ring2),
            health: 100,
        };
        assert_eq!(player.equipment_cost(), 22);
    }

    #[test]
    fn player_beats_test_1() {
        let boss = Boss {
            health: 12,
            damage: 7,
            armor: 2,
        };
        let player = Player {
            weapon: Weapon { cost: 5, damage: 5 },
            armor: Some(Armor { cost: 5, armor: 5 }),
            right_ring: None,
            left_ring: None,
            health: 8,
        };
        assert!(&player.beats(&boss));
    }

    #[test]
    fn boss_parse_test_1() {
        let lines = ["Hit Points: 123", "Damage: 22", "Armor: 5"];
        let correct = Boss {
            health: 123,
            damage: 22,
            armor: 5,
        };
        assert_eq!(Boss::parse(&lines).unwrap(), correct);
    }

    #[test]
    fn shop_lowest_cost_to_beat_test_1() {
        let shop = Shop {
            weapons: vec![Weapon { cost: 6, damage: 7 }],
            armor: vec![Armor { cost: 5, armor: 3 }],
            rings: vec![Ring {
                cost: 3,
                damage: 1,
                armor: 2,
            }],
        };
        let boss = Boss {
            health: 100,
            damage: 0,
            armor: 2,
        };
        assert_eq!(shop.lowest_cost_to_beat(100, &boss), Some(6));
    }

    #[test]
    fn shop_lowest_cost_to_beat_test_2() {
        let shop = Shop {
            weapons: vec![Weapon { cost: 6, damage: 7 }],
            armor: vec![Armor { cost: 5, armor: 3 }],
            rings: vec![Ring {
                cost: 11,
                damage: 1,
                armor: 2,
            }],
        };
        let boss = Boss {
            health: 100,
            damage: 3,
            armor: 2,
        };
        assert_eq!(shop.lowest_cost_to_beat(10, &boss), Some(11));
    }
}
