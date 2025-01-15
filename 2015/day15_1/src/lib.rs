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
}
