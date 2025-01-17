use regex::Regex;
use sscanf::sscanf;

#[derive(Debug, PartialEq, Default)]
struct Dogs {
    samoyeds: Option<u64>,
    pomeranians: Option<u64>,
    akitas: Option<u64>,
    vizslas: Option<u64>,
}

#[derive(Debug, PartialEq, Default)]
pub struct Sue {
    number: u64,
    children: Option<u64>,
    cats: Option<u64>,
    dogs: Dogs,
    goldfish: Option<u64>,
    trees: Option<u64>,
    cars: Option<u64>,
    perfumes: Option<u64>,
}

impl Sue {
    pub fn new(line: &str) -> Result<Sue, String> {
        let (number, items) = match sscanf!(line, "Sue {u64}: {String}") {
            Ok(p) => p,
            Err(_) => return Err(format!("failed to parse the line: {line}")),
        };
        let mut sue = Sue::default();
        sue.number = number;
        let items_regex = Regex::new(r"([a-z]+: \d+)").unwrap();
        let items_parsed = items_regex.find_iter(&items);
        for item in items_parsed {
            let (item, count) = match sscanf!(item.as_str(), "{String}: {u64}") {
                Ok(p) => p,
                Err(_) => return Err(format!("failed to parse the item: {}", item.as_str())),
            };
            match item.as_str() {
                "children" => sue.children = Some(count),
                "cats" => sue.cats = Some(count),
                "samoyeds" => sue.dogs.samoyeds = Some(count),
                "pomeranians" => sue.dogs.pomeranians = Some(count),
                "akitas" => sue.dogs.akitas = Some(count),
                "vizslas" => sue.dogs.vizslas = Some(count),
                "goldfish" => sue.goldfish = Some(count),
                "trees" => sue.trees = Some(count),
                "cars" => sue.cars = Some(count),
                "perfumes" => sue.perfumes = Some(count),
                _ => return Err(format!("invalid item name: {}", item.as_str())),
            }
        }
        Ok(sue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sue_new_test_1() {
        let sue = Sue::new("Sue 1234: perfumes: 2, children: 9, pomeranians: 4");
        let correct = Sue {
            number: 1234,
            perfumes: Some(2),
            children: Some(9),
            dogs: Dogs {
                pomeranians: Some(4),
                ..Default::default()
            },
            ..Default::default()
        };
        assert_eq!(sue, Ok(correct));
    }

    #[test]
    fn sue_new_test_2() {
        let sue = Sue::new("Sue 1234: elephants: 4, vizslas: 2, trees: 1");
        assert_eq!(sue, Err("invalid item name: elephants".to_owned()));
    }

    #[test]
    fn sue_new_test_3() {
        let sue = Sue::new("Sue 1234");
        assert_eq!(sue, Err("failed to parse the line: Sue 1234".to_owned()));
    }
}
