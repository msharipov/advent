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

enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn cw(&self) -> Self {
        use Direction::*;
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn ccw(&self) -> Self {
        self.cw().cw().cw()
    }
}

pub fn walk(moves: &[Move]) -> (i64, i64) {
    let mut position: (i64, i64) = (0, 0);
    let mut direction = Direction::North;
    for m in moves {
        let distance = match m {
            Move::R(len) => {
                direction = direction.cw();
                *len as i64
            }
            Move::L(len) => {
                direction = direction.ccw();
                *len as i64
            }
        };
        match direction {
            Direction::North => {
                position.1 += distance;
            }
            Direction::East => {
                position.0 += distance;
            }
            Direction::South => {
                position.1 -= distance;
            }
            Direction::West => {
                position.0 -= distance;
            }
        }
    }
    position
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

    #[test]
    fn walk_test_1() {
        let line = "L3, R5, L1, L1, L8";
        let moves = parse_moves(&line).unwrap();
        let final_pos = walk(&moves);
        assert_eq!(final_pos, (4, 4));
    }
}
