use regex::Regex;

#[derive(Debug, PartialEq)]
pub enum Part {
    RTG(String),
    Chip(String),
}

type Floors = [Vec<Part>; 4];

pub fn parse_floors(lines: &[&str]) -> Result<Floors, String> {
    if lines.len() != 4 {
        return Err("too many lines".to_owned());
    }
    let mut floors = Floors::default();
    let generator_regex = Regex::new(r"[a-z]+ generator").unwrap();
    let chip_regex = Regex::new(r"[a-z]+-compatible microchip").unwrap();
    for (floor, line) in lines.iter().enumerate() {
        for m in generator_regex.find_iter(line) {
            let kind = m.as_str().split(' ').next().unwrap();
            floors[floor].push(Part::RTG(kind.to_owned()));
        }
        for m in chip_regex.find_iter(line) {
            let kind = m.as_str().split('-').next().unwrap();
            floors[floor].push(Part::Chip(kind.to_owned()));
        }
    }
    Ok(floors)
}

fn is_valid(floors: &Floors) -> bool {
    for floor in floors {
        for part in floor {
            if let Part::Chip(element) = part {
                if !floor.contains(&Part::RTG(element.to_owned())) {
                    if floor
                        .iter()
                        .any(|other_part| matches!(other_part, Part::RTG(_)))
                    {
                        return false;
                    }
                }
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_floors_test_1() {
        let lines = [
            "The first floor contains a iron generator.",
            "The second floor contains a iron-compatible microchip and a xenon-compatible microchip.",
            "The third floor contains nothing relevant.",
            "The fourth floor contains a xenon generator.",
        ];
        let correct: Floors = [
            vec![Part::RTG("iron".to_owned())],
            vec![
                Part::Chip("iron".to_owned()),
                Part::Chip("xenon".to_owned()),
            ],
            vec![],
            vec![Part::RTG("xenon".to_owned())],
        ];
        assert_eq!(correct, parse_floors(&lines).unwrap());
    }

    #[test]
    fn is_valid_test_1() {
        let floors: Floors = [
            vec![
                Part::RTG("helium".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ],
            vec![],
            vec![
                Part::Chip("xenon".to_owned()),
                Part::Chip("iron".to_owned()),
            ],
            vec![Part::RTG("iron".to_owned())],
        ];
        assert!(is_valid(&floors));
    }

    #[test]
    fn is_valid_test_2() {
        let floors: Floors = [
            vec![
                Part::RTG("iron".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ],
            vec![],
            vec![
                Part::Chip("xenon".to_owned()),
                Part::Chip("iron".to_owned()),
            ],
            vec![Part::RTG("helium".to_owned())],
        ];
        assert!(!is_valid(&floors));
    }
}
