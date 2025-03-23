#[derive(Debug, PartialEq)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

pub fn moves_from_line(line: &str) -> Result<Vec<Move>, String> {
    use Move::{Down, Left, Right, Up};
    let mut moves = vec![];
    for c in line.chars() {
        match c {
            'U' => moves.push(Up),
            'R' => moves.push(Right),
            'D' => moves.push(Down),
            'L' => moves.push(Left),
            _ => return Err("invalid character '{c}'".to_owned()),
        }
    }
    Ok(moves)
}

pub fn get_code(lines: &[&str]) -> Result<String, String> {
    let mut code = String::default();
    let mut kp = Keypad::default();
    for line in lines {
        let moves = moves_from_line(line)?;
        for m in moves {
            use Move::*;
            match m {
                Up => kp.up(),
                Right => kp.right(),
                Down => kp.down(),
                Left => kp.left(),
            }
        }
        code.push(kp.0);
    }
    Ok(code)
}

pub struct Keypad(char);

impl Default for Keypad {
    fn default() -> Self {
        Self('7')
    }
}

impl Keypad {
    pub fn up(&mut self) {
        self.0 = match self.0 {
            '3' => '1',
            '6' => '2',
            '7' => '3',
            '8' => '4',
            'A' => '6',
            'B' => '7',
            'C' => '8',
            'D' => 'B',
            _ => self.0,
        };
    }

    pub fn down(&mut self) {
        self.0 = match self.0 {
            '1' => '3',
            '2' => '6',
            '3' => '7',
            '4' => '8',
            '6' => 'A',
            '7' => 'B',
            '8' => 'C',
            'B' => 'D',
            _ => self.0,
        };
    }

    pub fn left(&mut self) {
        self.0 = match self.0 {
            '3' => '2',
            '4' => '3',
            '6' => '5',
            '7' => '6',
            '8' => '7',
            '9' => '8',
            'B' => 'A',
            'C' => 'B',
            _ => self.0,
        };
    }

    pub fn right(&mut self) {
        self.0 = match self.0 {
            '2' => '3',
            '3' => '4',
            '5' => '6',
            '6' => '7',
            '7' => '8',
            '8' => '9',
            'A' => 'B',
            'B' => 'C',
            _ => self.0,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn keypad_move_test_1() {
        let mut kp = Keypad::default();
        kp.up();
        kp.up();
        kp.left();
        assert_eq!(kp.0, '1');
        kp.down();
        kp.left();
        kp.left();
        kp.right();
        kp.right();
        assert_eq!(kp.0, '4');
        kp.down();
        kp.left();
        kp.down();
        assert_eq!(kp.0, 'B');
        kp.left();
        kp.left();
        kp.up();
        kp.left();
        assert_eq!(kp.0, '5');
    }

    #[test]
    fn moves_from_lines_test_1() {
        use Move::*;
        let line = "ULLDLRD";
        let correct = vec![Up, Left, Left, Down, Left, Right, Down];
        assert_eq!(moves_from_line(&line), Ok(correct));
    }

    #[test]
    fn get_code_test_1() {
        let lines = ["UUR", "DRDLU", "DLLR", "RRDDL"];
        assert_eq!(get_code(&lines), Ok("136B".to_owned()));
    }
}
