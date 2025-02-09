#[derive(Default)]
pub struct Weapon {
    cost: i64,
    damage: i64,
}

#[derive(Default)]
pub struct Armor {
    cost: i64,
    armor: i64,
}

#[derive(Default)]
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
}

impl Player {
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
        };
        assert_eq!(player.armor(), 11);
    }
}
