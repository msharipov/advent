use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Screen {
    pixels: [[bool; 50]; 6],
}

impl Default for Screen {
    fn default() -> Self {
        Self {
            pixels: [[false; 50]; 6],
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

impl FromStr for Instruction {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, sscanf::Error> {
        if let Ok((a, b)) = sscanf!(s, "rect {usize}x{usize}") {
            return Ok(Instruction::Rect(a, b));
        }
        if let Ok((a, b)) = sscanf!(s, "rotate row y={usize} by {usize}") {
            return Ok(Instruction::RotateRow(a, b));
        }
        if let Ok((a, b)) = sscanf!(s, "rotate column x={usize} by {usize}") {
            return Ok(Instruction::RotateCol(a, b));
        }
        Err(sscanf::Error::MatchFailed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_from_str_test_1() {
        let line = "rect 5x6";
        assert_eq!(
            line.parse::<Instruction>().unwrap(),
            Instruction::Rect(5, 6)
        );
    }

    #[test]
    fn instruction_from_str_test_2() {
        let line = "rotate row y=5 by 13";
        assert_eq!(
            line.parse::<Instruction>().unwrap(),
            Instruction::RotateRow(5, 13)
        );
    }

    #[test]
    fn instruction_from_str_test_3() {
        let line = "rotate column x=22 by 2";
        assert_eq!(
            line.parse::<Instruction>().unwrap(),
            Instruction::RotateCol(22, 2)
        );
    }

    #[test]
    fn instruction_from_str_test_4() {
        let line = "rotate column y=22 by 2";
        assert!(line.parse::<Instruction>().is_err());
    }
}
