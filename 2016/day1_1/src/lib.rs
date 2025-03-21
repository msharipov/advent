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

pub fn parse_moves(line: &str) -> Result<Vec<Move>, sscanf::Error> {
    line.split(',')
        .map(str::trim)
        .map(str::parse::<Move>)
        .collect()
}

pub fn taxicab_distance(point_a: (i64, i64), point_b: (i64, i64)) -> u64 {
    point_a.0.abs_diff(point_b.0) + point_a.1.abs_diff(point_b.1)
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

    #[test]
    fn parse_moves_test_1() {
        use Move::{L, R};
        let line = "R3, L8, L2, R4";
        let correct = vec![R(3), L(8), L(2), R(4)];
        assert_eq!(parse_moves(&line).unwrap(), correct);
    }

    #[test]
    fn taxicab_distance_test_1() {
        assert_eq!(taxicab_distance((-8, 3), (1, -1)), 13);
    }
}
