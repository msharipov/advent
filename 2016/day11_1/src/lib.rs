use std::collections::BTreeSet;

use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Part {
    RTG(String),
    Chip(String),
}

type Floors = [BTreeSet<Part>; 4];

#[derive(Debug, PartialEq)]
struct State {
    elevator: usize,
    floors: Floors,
}

fn is_valid_floor(floor: &BTreeSet<Part>) -> bool {
    for part in floor {
        if let Part::Chip(element) = part {
            let has_matching_rtg = floor.contains(&Part::RTG(element.to_owned()));
            let has_other_rtgs = floor
                .iter()
                .any(|other_part| matches!(other_part, Part::RTG(_)));
            if !has_matching_rtg && has_other_rtgs {
                return false;
            }
        }
    }
    true
}

impl State {
    fn new(elevator: usize, floors: Floors) -> State {
        State { elevator, floors }
    }

    fn try_single_move(&self, dest: usize, part: &Part) -> Option<State> {
        if dest > 3 || dest.abs_diff(self.elevator) != 1 {
            return None;
        }
        let mut floors = self.floors.clone();
        let part = floors[self.elevator].take(part)?;
        floors[dest].insert(part);
        if is_valid_floor(&floors[dest]) {
            Some(State::new(dest, floors))
        } else {
            None
        }
    }
}

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
            floors[floor].insert(Part::RTG(kind.to_owned()));
        }
        for m in chip_regex.find_iter(line) {
            let kind = m.as_str().split('-').next().unwrap();
            floors[floor].insert(Part::Chip(kind.to_owned()));
        }
    }
    Ok(floors)
}

fn is_valid(floors: &Floors) -> bool {
    floors.iter().all(|f| is_valid_floor(f))
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
            BTreeSet::from_iter([Part::RTG("iron".to_owned())]),
            BTreeSet::from_iter([
                Part::Chip("iron".to_owned()),
                Part::Chip("xenon".to_owned()),
            ]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::RTG("xenon".to_owned())]),
        ];
        assert_eq!(correct, parse_floors(&lines).unwrap());
    }

    #[test]
    fn is_valid_test_1() {
        let floors: Floors = [
            BTreeSet::from_iter([
                Part::RTG("helium".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([
                Part::Chip("xenon".to_owned()),
                Part::Chip("iron".to_owned()),
            ]),
            BTreeSet::from_iter([Part::RTG("iron".to_owned())]),
        ];
        assert!(is_valid(&floors));
    }

    #[test]
    fn is_valid_test_2() {
        let floors: Floors = [
            BTreeSet::from_iter([
                Part::RTG("iron".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([
                Part::Chip("xenon".to_owned()),
                Part::Chip("iron".to_owned()),
            ]),
            BTreeSet::from_iter([Part::RTG("helium".to_owned())]),
        ];
        assert!(!is_valid(&floors));
    }

    #[test]
    fn try_single_move_test_1() {
        let floors: Floors = [
            BTreeSet::from_iter([
                Part::RTG("helium".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([
                Part::Chip("xenon".to_owned()),
                Part::Chip("iron".to_owned()),
            ]),
            BTreeSet::from_iter([Part::RTG("iron".to_owned())]),
        ];
        let state = State::new(1, floors);
        assert_eq!(
            state.try_single_move(0, &Part::RTG("neon".to_owned())),
            None
        );
    }

    #[test]
    fn try_single_move_test_2() {
        let floors: Floors = [
            BTreeSet::from_iter([
                Part::RTG("helium".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([
                Part::Chip("xenon".to_owned()),
                Part::Chip("iron".to_owned()),
            ]),
            BTreeSet::from_iter([Part::RTG("iron".to_owned())]),
        ];
        let state = State::new(0, floors);
        assert_eq!(
            state.try_single_move(2, &Part::Chip("helium".to_owned())),
            None
        );
    }

    #[test]
    fn try_single_move_test_3() {
        let floors: Floors = [
            BTreeSet::from_iter([
                Part::RTG("helium".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([
                Part::Chip("xenon".to_owned()),
                Part::Chip("iron".to_owned()),
            ]),
            BTreeSet::from_iter([Part::RTG("iron".to_owned())]),
        ];
        let state = State::new(3, floors);
        assert_eq!(
            state.try_single_move(2, &Part::RTG("iron".to_owned())),
            None
        );
    }

    #[test]
    fn try_single_move_test_4() {
        let floors: Floors = [
            BTreeSet::from_iter([
                Part::RTG("helium".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([
                Part::Chip("xenon".to_owned()),
                Part::Chip("iron".to_owned()),
            ]),
            BTreeSet::from_iter([Part::RTG("iron".to_owned())]),
        ];
        let state = State::new(2, floors);
        let new_floors = [
            BTreeSet::from_iter([
                Part::RTG("helium".to_owned()),
                Part::Chip("helium".to_owned()),
                Part::RTG("xenon".to_owned()),
            ]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip("xenon".to_owned())]),
            BTreeSet::from_iter([Part::Chip("iron".to_owned()), Part::RTG("iron".to_owned())]),
        ];
        let new_state = State::new(3, new_floors);
        assert_eq!(
            state.try_single_move(3, &Part::Chip("iron".to_owned())),
            Some(new_state)
        );
    }
}
