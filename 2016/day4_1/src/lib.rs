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
        let (name, id, checksum) = sscanf!(s, "{:/[a-z\\-]+/}{u32}[{String}]", String)?;
        Ok(Room { name, id, checksum })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn room_parse_test_1() {
        let correct = Room {
            name: "abc-def-ghik-lm-".to_owned(),
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
}
