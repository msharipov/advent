use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Operation {
    SwapPositions(usize, usize),
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
            return Ok(Operation::SwapPositions(x, y));
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

#[derive(Debug, PartialEq)]
pub enum OperationErr {
    OutOfBounds { index: usize },
    LetterNotFound { letter: char },
}

fn swap_positions(s: &str, pos_x: usize, pos_y: usize) -> Result<String, OperationErr> {
    let mut char_vec: Vec<_> = s.chars().collect();
    if pos_x >= char_vec.len() {
        return Err(OperationErr::OutOfBounds { index: pos_x });
    }
    if pos_y >= char_vec.len() {
        return Err(OperationErr::OutOfBounds { index: pos_y });
    }
    char_vec.swap(pos_x, pos_y);
    Ok(char_vec.iter().collect())
}

fn swap_letters(s: &str, letter_x: char, letter_y: char) -> Result<String, OperationErr> {
    let mut char_vec: Vec<_> = s.chars().collect();
    let index_x = match char_vec.iter().position(|&c| c == letter_x) {
        Some(pos) => pos,
        None => return Err(OperationErr::LetterNotFound { letter: letter_x }),
    };
    let index_y = match char_vec.iter().position(|&c| c == letter_y) {
        Some(pos) => pos,
        None => return Err(OperationErr::LetterNotFound { letter: letter_y }),
    };
    char_vec.swap(index_x, index_y);
    Ok(char_vec.iter().collect())
}

fn rotate_left(s: &str, distance: usize) -> String {
    let mut char_vec: Vec<_> = s.chars().collect();
    char_vec.rotate_left(distance);
    char_vec.iter().collect()
}

fn rotate_right(s: &str, distance: usize) -> String {
    let mut char_vec: Vec<_> = s.chars().collect();
    char_vec.rotate_right(distance);
    char_vec.iter().collect()
}

fn rotate_based_on_letter(s: &str, letter: char) -> Result<String, OperationErr> {
    let mut char_vec: Vec<_> = s.chars().collect();
    let index = match char_vec.iter().position(|&c| c == letter) {
        Some(i) => i,
        None => return Err(OperationErr::LetterNotFound { letter }),
    };
    char_vec.rotate_right(1 + index);
    if index >= 4 {
        char_vec.rotate_right(1);
    }
    Ok(char_vec.iter().collect())
}

fn reverse(s: &str, pos_x: usize, pos_y: usize) -> Result<String, OperationErr> {
    let mut char_vec: Vec<_> = s.chars().collect();
    if pos_x >= char_vec.len() {
        return Err(OperationErr::OutOfBounds { index: pos_x });
    }
    if pos_y >= char_vec.len() {
        return Err(OperationErr::OutOfBounds { index: pos_y });
    }
    char_vec[pos_x..=pos_y].reverse();
    Ok(char_vec.iter().collect())
}

fn move_position(s: &str, initial: usize, final_pos: usize) -> Result<String, OperationErr> {
    let mut char_vec: Vec<_> = s.chars().collect();
    if initial >= char_vec.len() {
        return Err(OperationErr::OutOfBounds { index: initial });
    }
    if final_pos >= char_vec.len() {
        return Err(OperationErr::OutOfBounds { index: final_pos });
    }
    let letter = char_vec.remove(initial);
    char_vec.insert(final_pos, letter);
    Ok(char_vec.iter().collect())
}

type ApplyOperationErr = (String, Operation, OperationErr);

pub fn apply_operations(s: &str, ops: &[Operation]) -> Result<String, ApplyOperationErr> {
    let mut s = s.to_owned();
    for op in ops {
        let result = match op {
            Operation::SwapPositions(x, y) => swap_positions(&s, *x, *y),
            Operation::SwapLetters(x, y) => swap_letters(&s, *x, *y),
            Operation::RotateLeftFixed(dist) => Ok(rotate_left(&s, *dist)),
            Operation::RotateRightFixed(dist) => Ok(rotate_right(&s, *dist)),
            Operation::RotateBasedOnLetter(c) => rotate_based_on_letter(&s, *c),
            Operation::Reverse(x, y) => reverse(&s, *x, *y),
            Operation::Move(x, y) => move_position(&s, *x, *y),
        };
        match result {
            Ok(new_s) => s = new_s,
            Err(e) => return Err((s, op.to_owned(), e)),
        };
    }
    Ok(s)
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
            Operation::SwapPositions(4, 0),
            Operation::SwapLetters('d', 'b'),
            Operation::RotateLeftFixed(1),
            Operation::Move(1, 4),
            Operation::RotateBasedOnLetter('b'),
            Operation::Reverse(6, 12),
        ];
        assert_eq!(correct, parse_instructions(&lines).unwrap());
    }

    #[test]
    fn swap_positions_test_1() {
        let s = "abcdefgh";
        assert_eq!(swap_positions(s, 7, 2), Ok("abhdefgc".to_owned()));
    }

    #[test]
    fn swap_positions_test_2() {
        let s = "abcdefgh";
        assert_eq!(
            swap_positions(s, 3, 9),
            Err(OperationErr::OutOfBounds { index: 9 })
        );
    }

    #[test]
    fn swap_positions_test_3() {
        let s = "abcdefgh";
        assert_eq!(
            swap_positions(s, 11, 5),
            Err(OperationErr::OutOfBounds { index: 11 })
        );
    }

    #[test]
    fn swap_letters_test_1() {
        let s = "abcdefgh";
        assert_eq!(swap_letters(s, 'b', 'e'), Ok("aecdbfgh".to_owned()));
    }

    #[test]
    fn swap_letters_test_2() {
        let s = "abcdefgh";
        assert_eq!(
            swap_letters(s, 'b', 'w'),
            Err(OperationErr::LetterNotFound { letter: 'w' })
        );
    }

    #[test]
    fn swap_letters_test_3() {
        let s = "abcdefgh";
        assert_eq!(
            swap_letters(s, 'p', 'g'),
            Err(OperationErr::LetterNotFound { letter: 'p' })
        );
    }

    #[test]
    fn rotate_left_test_1() {
        let s = "abcdefgh";
        assert_eq!(rotate_left(s, 2), "cdefghab");
    }

    #[test]
    fn rotate_right_test_1() {
        let s = "abcdefgh";
        assert_eq!(rotate_right(s, 3), "fghabcde");
    }

    #[test]
    fn rotate_based_on_letter_test_1() {
        let s = "abcdefgh";
        assert_eq!(rotate_based_on_letter(s, 'b'), Ok("ghabcdef".to_owned()));
    }

    #[test]
    fn rotate_based_on_letter_test_2() {
        let s = "abcdefgh";
        assert_eq!(rotate_based_on_letter(s, 'f'), Ok("bcdefgha".to_owned()));
    }

    #[test]
    fn rotate_based_on_letter_test_3() {
        let s = "abcdefgh";
        assert_eq!(
            rotate_based_on_letter(s, 'q'),
            Err(OperationErr::LetterNotFound { letter: 'q' })
        );
    }

    #[test]
    fn reverse_test_1() {
        let s = "abcdefgh";
        assert_eq!(reverse(s, 0, 4), Ok("edcbafgh".to_owned()));
    }

    #[test]
    fn reverse_test_2() {
        let s = "abcdefgh";
        assert_eq!(
            reverse(s, 11, 4),
            Err(OperationErr::OutOfBounds { index: 11 })
        );
    }

    #[test]
    fn reverse_test_3() {
        let s = "abcdefgh";
        assert_eq!(
            reverse(s, 1, 10),
            Err(OperationErr::OutOfBounds { index: 10 })
        );
    }

    #[test]
    fn move_position_test_1() {
        let s = "abcdefgh";
        assert_eq!(move_position(s, 6, 0), Ok("gabcdefh".to_owned()))
    }

    #[test]
    fn move_position_test_2() {
        let s = "abcdefgh";
        assert_eq!(move_position(s, 1, 4), Ok("acdebfgh".to_owned()))
    }

    #[test]
    fn move_position_test_3() {
        let s = "abcdefgh";
        assert_eq!(move_position(s, 5, 5), Ok("abcdefgh".to_owned()))
    }

    #[test]
    fn move_position_test_4() {
        let s = "abcdefgh";
        assert_eq!(
            move_position(s, 9, 5),
            Err(OperationErr::OutOfBounds { index: 9 })
        )
    }

    #[test]
    fn move_position_test_5() {
        let s = "abcdefgh";
        assert_eq!(
            move_position(s, 6, 12),
            Err(OperationErr::OutOfBounds { index: 12 })
        )
    }

    #[test]
    fn apply_operations_test_1() {
        use Operation::*;
        let s = "abcde";
        let ops = [
            SwapPositions(4, 0),
            SwapLetters('d', 'b'),
            Reverse(0, 4),
            RotateLeftFixed(1),
            Move(1, 4),
            Move(3, 0),
            RotateBasedOnLetter('b'),
            RotateBasedOnLetter('d'),
        ];
        assert_eq!(apply_operations(s, &ops), Ok("decab".to_owned()));
    }
}
