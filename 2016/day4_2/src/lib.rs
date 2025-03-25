use itertools::Itertools;
use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Room {
    name: String,
    id: u32,
    checksum: String,
}

impl FromStr for Room {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, id, checksum) = sscanf!(s, "{:/[a-z\\-]+/}-{u32}[{String}]", String)?;
        Ok(Room { name, id, checksum })
    }
}

impl Room {
    fn checksum(&self) -> String {
        let mut counts = self.name.chars().counts();
        counts.remove(&'-');
        counts
            .iter()
            .sorted_by(|a, b| match a.1.cmp(b.1) {
                std::cmp::Ordering::Equal => b.0.cmp(a.0),
                _ => a.1.cmp(b.1),
            })
            .rev()
            .take(5)
            .map(|(k, _)| k)
            .collect()
    }

    pub fn sector(&self) -> u32 {
        self.id
    }

    pub fn is_valid(&self) -> bool {
        self.checksum() == self.checksum
    }

    pub fn decrypt_name(&self) -> String {
        let mut decrypted = String::default();
        for c in self.name.chars() {
            decrypted.push(if c == '-' {
                ' '
            } else {
                char::from_u32((c as u32 - 'a' as u32 + self.id) % 26 + 'a' as u32).unwrap()
            });
        }
        decrypted
    }
}

pub fn parse_rooms(lines: &[&str]) -> Result<Vec<Room>, sscanf::Error> {
    lines.iter().map(|&line| line.parse::<Room>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_parse_test_1() {
        let correct = Room {
            name: "abc-def-ghik-lm".to_owned(),
            id: 1234,
            checksum: "abcde".to_owned(),
        };
        let line = "abc-def-ghik-lm-1234[abcde]";
        assert_eq!(line.parse::<Room>().unwrap(), correct);
    }

    #[test]
    fn room_parse_test_2() {
        let line = "asdf-avcv-dsf-[adfsv]";
        assert!(line.parse::<Room>().is_err());
    }

    #[test]
    fn room_checksum_test_1() {
        let room = Room {
            name: "abr-acad-abra-room".to_owned(),
            id: 123,
            checksum: "abcdef".to_owned(),
        };
        assert_eq!(room.checksum(), "arboc");
    }

    #[test]
    fn room_decrypt_test_1() {
        let room = "qzmt-zixmtkozy-ivhz-343[asadf]".parse::<Room>().unwrap();
        assert_eq!(room.decrypt_name(), "very encrypted name");
    }
}
