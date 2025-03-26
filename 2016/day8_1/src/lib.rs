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

impl Screen {
    fn rect(&mut self, a: usize, b: usize) {
        for x in 0..a {
            for y in 0..b {
                self.pixels[y][x] = true;
            }
        }
    }

    fn rotate_row(&mut self, y: usize, len: usize) {
        let old_row = self.pixels[y].clone();
        for x in 0..50 {
            self.pixels[y][(x + len) % 50] = old_row[x];
        }
    }

    fn rotate_col(&mut self, x: usize, len: usize) {
        let old_col = self.pixels.iter().map(|row| row[x]).collect::<Vec<bool>>();
        for y in 0..6 {
            self.pixels[(y + len) % 6][x] = old_col[y];
        }
    }

    pub fn apply_instruction(&mut self, inst: &Instruction) {
        match *inst {
            Instruction::Rect(a, b) => self.rect(a, b),
            Instruction::RotateRow(a, b) => self.rotate_row(a, b),
            Instruction::RotateCol(a, b) => self.rotate_col(a, b),
        };
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

    #[test]
    fn screen_rect_test_1() {
        let mut screen = Screen::default();
        screen.apply_instruction(&Instruction::Rect(8, 2));
        for x in 0..50 {
            for y in 0..6 {
                if x < 8 && y < 2 {
                    assert!(screen.pixels[y][x]);
                } else {
                    assert!(!screen.pixels[y][x]);
                }
            }
        }
    }

    #[test]
    fn screen_rotate_row_test_1() {
        let mut screen = Screen::default();
        screen.apply_instruction(&Instruction::Rect(8, 2));
        screen.apply_instruction(&Instruction::RotateRow(0, 3));
        for x in 0..50 {
            for y in 0..6 {
                if (y == 0 && x >= 3 && x < 11) || (y == 1 && x < 8) {
                    assert!(screen.pixels[y][x]);
                } else {
                    assert!(!screen.pixels[y][x]);
                }
            }
        }
    }

    #[test]
    fn screen_rotate_col_test_1() {
        let mut screen = Screen::default();
        screen.apply_instruction(&Instruction::Rect(8, 5));
        screen.apply_instruction(&Instruction::RotateCol(3, 3));
        for x in 0..50 {
            for y in 0..6 {
                if (x < 8 && x != 3 && y != 5) || (x == 3 && y != 2) {
                    assert!(screen.pixels[y][x]);
                } else {
                    assert!(!screen.pixels[y][x]);
                }
            }
        }
    }
}
