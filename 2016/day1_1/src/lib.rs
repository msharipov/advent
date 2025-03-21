use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Move {
    R(u64),
    L(u64),
}

impl FromStr for Move {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, len) = sscanf!(s, "{:/R|L/}{u64}", String)?;
        match dir.as_str() {
            "R" => Ok(Move::R(len)),
            "L" => Ok(Move::L(len)),
            _ => Err(sscanf::Error::MatchFailed),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_parse_test_1() {
        assert_eq!("R45".parse::<Move>().unwrap(), Move::R(45));
    }

    #[test]
    fn move_parse_test_2() {
        assert_eq!("L45".parse::<Move>().unwrap(), Move::L(45));
    }

    #[test]
    fn move_parse_test_3() {
        assert!("A45".parse::<Move>().is_err());
    }
}
