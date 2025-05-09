use regex::Regex;
use sscanf::sscanf;
use std::collections::{HashMap, HashSet};

type Replacements = HashMap<String, HashSet<Vec<String>>>;
type InvReplacements = HashMap<Vec<String>, String>;

pub fn parse_replacements<'a, I: Iterator<Item = &'a str>>(
    iter: &mut I,
) -> Result<Replacements, String> {
    let mut replacements = HashMap::new();
    loop {
        match iter.next() {
            None => return Err("premature input termination".to_owned()),
            Some(line) => {
                if line.is_empty() {
                    return Ok(replacements);
                } else {
                    let parsed = sscanf!(line, "{String} => {String}");
                    if parsed.is_err() {
                        return Err(format!("cannot parse {line}"));
                    }
                    let (key, val) = parsed.unwrap();
                    let val = sequence_from_line(&val);
                    match replacements.get_mut(&key) {
                        None => {
                            let mut set = HashSet::new();
                            set.insert(val);
                            replacements.insert(key, set);
                        }
                        Some(set) => {
                            set.insert(val);
                        }
                    }
                }
            }
        }
    }
}

pub fn invert_replacements(repl: &Replacements) -> InvReplacements {
    let mut inverse = HashMap::new();
    for (molecule, set) in repl {
        for replacement in set {
            inverse.insert(replacement.to_owned(), molecule.to_owned());
        }
    }
    inverse
}

fn sequence_from_line(sequence: &str) -> Vec<String> {
    let regex = Regex::new("[A-Z][a-z]?").unwrap();
    regex
        .find_iter(sequence)
        .map(|m| m.as_str().to_owned())
        .collect()
}

pub fn parse_sequence<'a>(iter: &mut impl Iterator<Item = &'a str>) -> Result<Vec<String>, String> {
    let sequence = match iter.next() {
        None => return Err("sequence is missing".to_owned()),
        Some(s) => s,
    };
    Ok(sequence_from_line(sequence))
}

pub fn count_new_sequences(seq: &[String], repl: &Replacements) -> usize {
    let mut sequences = HashSet::new();
    for (index, molecule) in seq.iter().enumerate() {
        let mut head = vec![];
        head.append(&mut seq[0..index].to_vec());
        let mut tail = vec![];
        tail.append(&mut seq[index + 1..].to_vec());
        if let Some(set) = repl.get(molecule) {
            for replacement in set {
                let mut combined = head.clone();
                combined.append(&mut replacement.clone());
                combined.append(&mut tail);
                sequences.insert(combined);
            }
        }
    }
    sequences.len()
}

#[derive(Debug)]
pub struct CannotInvert;

pub fn invert_first(seq: &mut Vec<String>, repl: &Replacements) -> Result<(), CannotInvert> {
    let mut inv_repl = invert_replacements(repl);
    let mut e_inversions = HashMap::new();
    for (k, v) in inv_repl.clone() {
        if v == "e" {
            e_inversions.insert(k.clone(), v);
            inv_repl.remove(&k);
        }
    }
    for inverse in &inv_repl {
        let (pattern, replacement) = inverse;
        let length = pattern.len();
        for (i, win) in seq.windows(length).enumerate() {
            if win == pattern {
                seq[i] = replacement.clone();
                seq.drain(i + 1..i + length);
                return Ok(());
            }
        }
    }
    for inverse in &e_inversions {
        let (pattern, replacement) = inverse;
        let length = pattern.len();
        for (i, win) in seq.windows(length).enumerate() {
            if win == pattern {
                seq[i] = replacement.clone();
                seq.drain(i + 1..i + length);
                return Ok(());
            }
        }
    }
    Err(CannotInvert)
}

pub fn count_inversions(seq: &mut Vec<String>, repl: &Replacements) -> Result<u64, CannotInvert> {
    let mut count = 0;
    while seq != &vec!["e".to_owned()] {
        if invert_first(seq, repl).is_err() {
            return Err(CannotInvert);
        }
        count += 1;
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_replacements_test_1() {
        let mut lines = ["H => HO", "H => OH", "O => HH", "", "abracadabra"]
            .iter()
            .copied();
        let parsed = parse_replacements(&mut lines).unwrap();
        let mut correct = HashMap::new();
        correct.insert(
            "H".to_owned(),
            HashSet::from([
                vec!["H".to_owned(), "O".to_owned()],
                vec!["O".to_owned(), "H".to_owned()],
            ]),
        );
        correct.insert(
            "O".to_owned(),
            HashSet::from([vec!["H".to_owned(), "H".to_owned()]]),
        );
        assert_eq!(correct, parsed);
        assert_eq!(lines.next(), Some("abracadabra"));
    }

    #[test]
    fn parse_sequence_test_1() {
        let mut lines = ["H => HO", "H => OH", "O => HH", "", "HAsTeOHCaHe"]
            .iter()
            .copied();
        let _ = parse_replacements(&mut lines).unwrap();
        let parsed = parse_sequence(&mut lines).unwrap();
        assert_eq!(parsed, vec!["H", "As", "Te", "O", "H", "Ca", "He"]);
    }

    #[test]
    fn parse_sequence_test_2() {
        let mut lines = ["H => HO", "H => OH", "O => HH", ""].iter().copied();
        let _ = parse_replacements(&mut lines).unwrap();
        let parsed = parse_sequence(&mut lines);
        assert!(parsed.is_err());
    }

    #[test]
    fn count_new_sequences_test_1() {
        let mut lines = ["O => OO", "H => OH", "", "HOO"].iter().copied();
        let repl = parse_replacements(&mut lines).unwrap();
        let seq = parse_sequence(&mut lines).unwrap();
        // OHOO, HOOO
        assert_eq!(count_new_sequences(&seq, &repl), 2);
    }

    #[test]
    fn count_new_sequences_test_2() {
        let mut lines = ["H => HO", "H => OH", "O => HH", "", "HAsTeOHCaHe"]
            .iter()
            .copied();
        let repl = parse_replacements(&mut lines).unwrap();
        let seq = parse_sequence(&mut lines).unwrap();
        assert_eq!(count_new_sequences(&seq, &repl), 5);
    }

    #[test]
    fn invert_replacements_test_1() {
        let mut lines = ["H => HO", "H => OH", "O => HH", "", "HAsTeOHCaHe"]
            .iter()
            .copied();
        let repl = parse_replacements(&mut lines).unwrap();
        let inv = invert_replacements(&repl);
        let correct = {
            let mut map = HashMap::new();
            map.insert(vec!["H".to_owned(), "O".to_owned()], "H".to_owned());
            map.insert(vec!["O".to_owned(), "H".to_owned()], "H".to_owned());
            map.insert(vec!["H".to_owned(), "H".to_owned()], "O".to_owned());
            map
        };
        assert_eq!(correct, inv);
    }

    #[test]
    fn sequence_from_line_test_1() {
        let molecule = "ArHOTeTiCl";
        let correct = vec!["Ar", "H", "O", "Te", "Ti", "Cl"];
        assert_eq!(sequence_from_line(molecule), correct);
    }

    #[test]
    fn invert_first_test_1() {
        let mut lines = [
            "Ar => NeCl",
            "O => HO",
            "O => ArTi",
            "e => O",
            "",
            "HNeClTi",
        ]
        .into_iter();
        let repl = parse_replacements(&mut lines).unwrap();
        let mut seq = parse_sequence(&mut lines).unwrap();
        assert!(invert_first(&mut seq, &repl).is_ok());
        assert_eq!(
            &seq,
            &vec!["H".to_owned(), "Ar".to_owned(), "Ti".to_owned()]
        );
        assert!(invert_first(&mut seq, &repl).is_ok());
        assert_eq!(&seq, &vec!["H".to_owned(), "O".to_owned()]);
        assert!(invert_first(&mut seq, &repl).is_ok());
        assert_eq!(&seq, &vec!["O".to_owned()]);
        assert!(invert_first(&mut seq, &repl).is_ok());
        assert_eq!(&seq, &vec!["e".to_owned()]);
        assert!(invert_first(&mut seq, &repl).is_err());
    }

    #[test]
    fn count_inversions_test_1() {
        let mut lines = [
            "Ar => NeCl",
            "O => HO",
            "O => ArTi",
            "e => O",
            "",
            "HNeClTi",
        ]
        .into_iter();
        let repl = parse_replacements(&mut lines).unwrap();
        let mut seq = parse_sequence(&mut lines).unwrap();
        assert_eq!(count_inversions(&mut seq, &repl).unwrap(), 4);
    }

    #[test]
    fn count_inversions_test_2() {
        let mut lines = [
            "Ar => NeCl",
            "O => HTe",
            "O => ArTi",
            "e => O",
            "",
            "HNeClTi",
        ]
        .into_iter();
        let repl = parse_replacements(&mut lines).unwrap();
        let mut seq = parse_sequence(&mut lines).unwrap();
        assert!(count_inversions(&mut seq, &repl).is_err());
    }
}
