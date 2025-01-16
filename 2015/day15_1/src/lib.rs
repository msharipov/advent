use std::cmp::max;

use sscanf::sscanf;

#[derive(Debug, PartialEq)]
pub struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    pub fn new(line: &str) -> Result<Ingredient, sscanf::Error> {
        let parsed = sscanf!(line, "{String}: capacity {i32}, durability {i32}, flavor {i32}, texture {i32}, calories {i32}")?;
        Ok(Ingredient {
            name: parsed.0,
            capacity: parsed.1,
            durability: parsed.2,
            flavor: parsed.3,
            texture: parsed.4,
            calories: parsed.5,
        })
    }
}

pub fn parse_ingredients(lines: &[&str]) -> Result<Vec<Ingredient>, sscanf::Error> {
    lines.iter().map(|line| Ingredient::new(line)).collect()
}

pub fn cookie_score(recipe: &[(Ingredient, i32)]) -> i32 {
    let total_capacity = max(
        recipe
            .iter()
            .map(|(ing, spoons)| ing.capacity * spoons)
            .sum(),
        0,
    );
    let total_durability = max(
        recipe
            .iter()
            .map(|(ing, spoons)| ing.durability * spoons)
            .sum(),
        0,
    );
    let total_flavor = max(
        recipe.iter().map(|(ing, spoons)| ing.flavor * spoons).sum(),
        0,
    );
    let total_texture = max(
        recipe
            .iter()
            .map(|(ing, spoons)| ing.texture * spoons)
            .sum(),
        0,
    );
    total_capacity * total_durability * total_flavor * total_texture
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ingredient_new_test_1() {
        assert_eq!(
            Ingredient::new(
                "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"
            )
            .unwrap(),
            Ingredient {
                name: "Butterscotch".to_owned(),
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            }
        )
    }

    #[test]
    fn cookie_score_test_1() {
        let recipe = [
            (
                Ingredient {
                    name: "Butterscotch".to_owned(),
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8,
                },
                20,
            ),
            (
                Ingredient {
                    name: "Caramel".to_owned(),
                    capacity: 3,
                    durability: 1,
                    flavor: 3,
                    texture: 1,
                    calories: 6,
                },
                60,
            ),
        ];
        assert_eq!(cookie_score(&recipe), 115_200_000);
    }

    #[test]
    fn cookie_score_test_2() {
        let recipe = [
            (
                Ingredient {
                    name: "Butterscotch".to_owned(),
                    capacity: -1,
                    durability: -2,
                    flavor: 6,
                    texture: 3,
                    calories: 8,
                },
                50,
            ),
            (
                Ingredient {
                    name: "Caramel".to_owned(),
                    capacity: 3,
                    durability: 1,
                    flavor: 3,
                    texture: 1,
                    calories: 6,
                },
                50,
            ),
        ];
        assert_eq!(cookie_score(&recipe), 0);
    }
}
