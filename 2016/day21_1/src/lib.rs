use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Operation {
    SwapPosition(usize, usize),
    SwapLetters(char, char),
    RotateLeftFixed(usize),
    RotateRightFixed(usize),
    RotateBasedOnLetter(char),
    Reverse(usize, usize),
    Move(usize, usize),
}

impl FromStr for Operation {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((x, y)) = sscanf!(s, "swap position {usize} with position {usize}") {
            return Ok(Operation::SwapPosition(x, y));
        }
        if let Ok((x, y)) = sscanf!(s, "swap letter {char} with letter {char}") {
            return Ok(Operation::SwapLetters(x, y));
        }
        if let Ok((x, _)) = sscanf!(s, "rotate left {usize} {:/step|steps/}", String) {
            return Ok(Operation::RotateLeftFixed(x));
        }
        if let Ok((x, _)) = sscanf!(s, "rotate right {usize} {:/step|steps/}", String) {
            return Ok(Operation::RotateRightFixed(x));
        }
        if let Ok(x) = sscanf!(s, "rotate based on position of letter {char}") {
            return Ok(Operation::RotateBasedOnLetter(x));
        }
        if let Ok((x, y)) = sscanf!(s, "reverse positions {usize} through {usize}") {
            return Ok(Operation::Reverse(x, y));
        }
        if let Ok((x, y)) = sscanf!(s, "move position {usize} to position {usize}") {
            return Ok(Operation::Move(x, y));
        }
        Err(sscanf::Error::MatchFailed)
    }
}

pub fn parse_instructions(lines: &[&str]) -> Result<Vec<Operation>, sscanf::Error> {
    lines.iter().map(|line| line.parse::<Operation>()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instructions_test_1() {
        let lines = [
            "swap position 4 with position 0",
            "swap letter d with letter b",
            "rotate left 1 step",
            "move position 1 to position 4",
            "rotate based on position of letter b",
            "reverse positions 6 through 12",
        ];
        let correct = vec![
            Operation::SwapPosition(4, 0),
            Operation::SwapLetters('d', 'b'),
            Operation::RotateLeftFixed(1),
            Operation::Move(1, 4),
            Operation::RotateBasedOnLetter('b'),
            Operation::Reverse(6, 12),
        ];
        assert_eq!(correct, parse_instructions(&lines).unwrap());
    }
}
