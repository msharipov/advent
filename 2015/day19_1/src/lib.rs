use sscanf::sscanf;
use std::collections::{HashMap, HashSet};

type Replacements = HashMap<String, HashSet<String>>;

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
                    let (key, val) = sscanf!(line, "{String} => {String}")
                        .expect(&format!("failed to parse {line}"));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_replacements_test_1() {
        let mut lines = ["H => HO", "H => OH", "O => HH", "", "abracadabra"].iter().copied();
        let parsed = parse_replacements(&mut lines).unwrap();
        let mut correct = HashMap::new();
        correct.insert("H".to_owned(), HashSet::from(["HO".to_owned(), "OH".to_owned()]));
        correct.insert("O".to_owned(), HashSet::from(["HH".to_owned()]));
        assert_eq!(correct, parsed);
        assert_eq!(lines.next(), Some("abracadabra"));
    }
}
