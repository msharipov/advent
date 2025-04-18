use std::{
    collections::{BTreeSet, HashMap, HashSet},
    thread,
};

use itertools::Itertools;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub enum Part {
    RTG(u8),
    Chip(u8),
}

type Floors = [BTreeSet<Part>; 4];

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
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
        if is_valid_floor(&floors[dest]) && is_valid_floor(&floors[self.elevator]) {
            Some(State::new(dest, floors))
        } else {
            None
        }
    }

    fn try_double_move(&self, dest: usize, parts: (&Part, &Part)) -> Option<State> {
        if dest > 3 || dest.abs_diff(self.elevator) != 1 {
            return None;
        }
        let (part1, part2) = parts;
        let mut floors = self.floors.clone();
        let part1 = floors[self.elevator].take(part1)?;
        let part2 = floors[self.elevator].take(part2)?;
        floors[dest].insert(part1);
        floors[dest].insert(part2);
        if is_valid_floor(&floors[dest]) && is_valid_floor(&floors[self.elevator]) {
            Some(State::new(dest, floors))
        } else {
            None
        }
    }

    fn adjacent_states(&self) -> Vec<State> {
        let mut adjacent = vec![];
        for part in &self.floors[self.elevator] {
            if self.elevator > 0 {
                if let Some(s) = self.try_single_move(self.elevator - 1, part) {
                    adjacent.push(s);
                }
            }
            if let Some(s) = self.try_single_move(self.elevator + 1, part) {
                adjacent.push(s);
            }
        }
        for pair in self.floors[self.elevator].iter().combinations(2) {
            let parts = (pair[0], pair[1]);
            if self.elevator > 0 {
                if let Some(s) = self.try_double_move(self.elevator - 1, parts) {
                    adjacent.push(s);
                }
            }
            if let Some(s) = self.try_double_move(self.elevator + 1, parts) {
                adjacent.push(s);
            }
        }
        adjacent
    }
}

pub fn parse_floors(lines: &[&str]) -> Result<Floors, String> {
    if lines.len() != 4 {
        return Err("too many lines".to_owned());
    }
    let mut floors = Floors::default();
    let mut elements = HashMap::new();
    let generator_regex = Regex::new(r"[a-z]+ generator").unwrap();
    let chip_regex = Regex::new(r"[a-z]+-compatible microchip").unwrap();
    for (floor, line) in lines.iter().enumerate() {
        for m in generator_regex.find_iter(line) {
            let kind = m.as_str().split(' ').next().unwrap().to_owned();
            let number = match elements.get(&kind) {
                Some(&num) => num,
                None => {
                    elements.insert(kind, elements.len());
                    elements.len() - 1
                }
            } as u8;
            floors[floor].insert(Part::RTG(number));
        }
        for m in chip_regex.find_iter(line) {
            let kind = m.as_str().split('-').next().unwrap().to_owned();
            let number = match elements.get(&kind) {
                Some(&num) => num,
                None => {
                    elements.insert(kind, elements.len());
                    elements.len() - 1
                }
            } as u8;
            floors[floor].insert(Part::Chip(number));
        }
    }
    Ok(floors)
}

fn final_state(floors: &Floors) -> State {
    let mut elements = HashSet::new();
    for floor in floors {
        for part in floor {
            match part {
                Part::RTG(el) => {
                    elements.insert(el.to_owned());
                }
                Part::Chip(el) => {
                    elements.insert(el.to_owned());
                }
            }
        }
    }
    let mut final_floors = [
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
        BTreeSet::new(),
    ];
    for element in elements {
        final_floors[3].insert(Part::Chip(element.to_owned()));
        final_floors[3].insert(Part::RTG(element.to_owned()));
    }
    State::new(3, final_floors)
}

pub fn least_steps_to_finish(floors: &Floors) -> Option<u64> {
    let mut count = 0u64;
    let final_state = final_state(floors);
    let initial_state = State::new(0, floors.clone());
    let mut horizon: HashSet<_> = HashSet::from_iter([initial_state.clone()]);
    let mut explored: HashMap<State, u64> = HashMap::from_iter([(initial_state, 0)]);
    while !horizon.contains(&final_state) {
        let mut new_horizon = HashSet::new();
        count += 1;
        let thread_count = 12;
        let chunk_size = horizon.len().div_ceil(thread_count);
        let chunks = horizon.into_iter().chunks(chunk_size);
        let chunks = chunks
            .into_iter()
            .map(|chunk| chunk.collect_vec())
            .collect_vec();
        let mut handles = vec![];
        for chunk in chunks.into_iter() {
            handles.push(thread::spawn(move || {
                chunk
                    .iter()
                    .map(move |state| state.adjacent_states())
                    .collect_vec()
            }))
        }
        for handle in handles {
            let new_states = handle.join().unwrap();
            for adjacent in new_states {
                for adj in adjacent {
                    if !explored.contains_key(&adj) {
                        explored.insert(adj.clone(), count);
                        new_horizon.insert(adj);
                    }
                }
            }
        }
        if new_horizon.is_empty() {
            return None;
        }
        horizon = new_horizon;
        println!("step {count}");
    }
    Some(count)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

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
            BTreeSet::from_iter([Part::RTG(0)]),
            BTreeSet::from_iter([Part::Chip(0), Part::Chip(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::RTG(1)]),
        ];
        assert_eq!(correct, parse_floors(&lines).unwrap());
    }

    #[test]
    fn is_valid_floor_test_1() {
        let floors: Floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        assert!(floors.iter().all(|f| is_valid_floor(f)));
    }

    #[test]
    fn is_valid_floor_test_2() {
        let floor = BTreeSet::from_iter([Part::RTG(0), Part::Chip(1), Part::RTG(2)]);
        assert!(!is_valid_floor(&floor));
    }

    #[test]
    fn try_single_move_test_1() {
        let floors: Floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state = State::new(1, floors);
        assert_eq!(state.try_single_move(0, &Part::RTG(4)), None);
    }

    #[test]
    fn try_single_move_test_2() {
        let floors: Floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state = State::new(0, floors);
        assert_eq!(state.try_single_move(2, &Part::Chip(0)), None);
    }

    #[test]
    fn try_single_move_test_3() {
        let floors: Floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state = State::new(3, floors);
        assert_eq!(state.try_single_move(2, &Part::RTG(2)), None);
    }

    #[test]
    fn try_single_move_test_4() {
        let floors: Floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state = State::new(2, floors);
        let new_floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1)]),
            BTreeSet::from_iter([Part::Chip(2), Part::RTG(2)]),
        ];
        let new_state = State::new(3, new_floors);
        assert_eq!(state.try_single_move(3, &Part::Chip(2)), Some(new_state));
    }

    #[test]
    fn try_single_move_test_5() {
        let floors: Floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state = State::new(0, floors);
        assert_eq!(state.try_single_move(1, &Part::RTG(0)), None);
    }

    #[test]
    fn try_double_move_test_1() {
        let floors: Floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state = State::new(0, floors);
        let new_floors = [
            BTreeSet::from_iter([Part::Chip(0)]),
            BTreeSet::from_iter([Part::RTG(0), Part::RTG(1)]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let new_state = State::new(1, new_floors);
        assert_eq!(
            state.try_double_move(1, (&Part::RTG(0), &Part::RTG(1))),
            Some(new_state)
        );
    }

    #[test]
    fn try_double_move_test_2() {
        let floors: Floors = [
            BTreeSet::from_iter([Part::RTG(0), Part::Chip(0), Part::RTG(1)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state = State::new(2, floors);
        assert_eq!(
            state.try_double_move(3, (&Part::Chip(1), &Part::Chip(2))),
            None
        );
    }

    #[test]
    fn adjacent_states_test_1() {
        let floors = [
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state = State::new(2, floors);
        let floors_1 = [
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1)]),
            BTreeSet::from_iter([Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state_1 = State::new(1, floors_1);
        let floors_2 = [
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(2)]),
            BTreeSet::from_iter([Part::Chip(1)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state_2 = State::new(1, floors_2);
        let floors_3 = [
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        let state_3 = State::new(1, floors_3);
        let floors_4 = [
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::Chip(1)]),
            BTreeSet::from_iter([Part::RTG(2), Part::Chip(2)]),
        ];
        let state_4 = State::new(3, floors_4);
        let mut correct = HashSet::new();
        correct.insert(state_1);
        correct.insert(state_2);
        correct.insert(state_3);
        correct.insert(state_4);
        let adjacent = HashSet::from_iter(state.adjacent_states());
        assert_eq!(correct, adjacent);
    }

    #[test]
    fn least_steps_to_finish_test_1() {
        let floors = [
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([Part::RTG(1)]),
            BTreeSet::from_iter([Part::Chip(1), Part::Chip(2)]),
            BTreeSet::from_iter([Part::RTG(2)]),
        ];
        assert_eq!(least_steps_to_finish(&floors), None);
    }

    #[test]
    fn least_steps_to_finish_test_2() {
        let floors = [
            BTreeSet::from_iter([Part::RTG(2), Part::Chip(2)]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([]),
            BTreeSet::from_iter([]),
        ];
        assert_eq!(least_steps_to_finish(&floors), Some(3));
    }
}
