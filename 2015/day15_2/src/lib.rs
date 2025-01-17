use itertools::Itertools;
use sscanf::sscanf;
use std::cmp::max;

#[derive(Debug, PartialEq, Clone)]
pub struct Ingredient {
    name: String,
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    pub fn new(line: &str) -> Result<Ingredient, sscanf::Error> {
        let parsed = sscanf!(line, "{String}: capacity {i64}, durability {i64}, flavor {i64}, texture {i64}, calories {i64}")?;
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

pub fn cookie_score(recipe: &[(Ingredient, i64)]) -> i64 {
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

fn count_calories(recipe: &[(Ingredient, i64)]) -> i64 {
    recipe
        .iter()
        .map(|(ing, spoons)| ing.calories * spoons)
        .sum()
}

pub fn best_score(ingredients: &[Ingredient], max_spoons: i64, calories: Option<i64>) -> i64 {
    let ing_count = ingredients.len();
    let mut best_score: i64 = 0;
    let mut cutoffs_vec = (0..=max_spoons).combinations(ing_count - 1);
    for mut cutoffs in &mut cutoffs_vec {
        cutoffs.sort();
        let mut amounts = vec![cutoffs[0]];
        for ing_amount in cutoffs.windows(2) {
            amounts.push(ing_amount[1] - ing_amount[0]);
        }
        amounts.push(max_spoons - cutoffs[cutoffs.len() - 1]);
        let recipe = ingredients
            .into_iter()
            .zip(amounts.iter())
            .map(|(ing, &amount)| (ing.clone(), amount.clone()))
            .collect::<Vec<_>>();
        if let Some(c) = calories {
            if c != count_calories(&recipe) {
                continue;
            }
        }
        let score = cookie_score(&recipe);
        best_score = max(score, best_score);
    }
    best_score
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

    #[test]
    fn count_calories_test_1() {
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
        assert_eq!(count_calories(&recipe), 700);
    }

    #[test]
    fn best_score_test_1() {
        let ingredients = [
            Ingredient {
                name: "Butterscotch".to_owned(),
                capacity: -1,
                durability: -2,
                flavor: 6,
                texture: 3,
                calories: 8,
            },
            Ingredient {
                name: "Caramel".to_owned(),
                capacity: 3,
                durability: 1,
                flavor: 3,
                texture: 1,
                calories: 6,
            },
        ];
        assert_eq!(best_score(&ingredients, 3, None), 729);
    }

    #[test]
    fn best_score_test_2() {
        let ingredients = [
            Ingredient {
                name: "Cinnamon".to_owned(),
                capacity: 2,
                durability: 1,
                flavor: 4,
                texture: 1,
                calories: 1,
            },
            Ingredient {
                name: "Caramel".to_owned(),
                capacity: 3,
                durability: 1,
                flavor: 3,
                texture: 1,
                calories: 6,
            },
        ];
        assert_eq!(best_score(&ingredients, 3, Some(8)), 693);
    }
}
