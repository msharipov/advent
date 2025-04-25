use std::str::FromStr;

pub enum Move {
    Up,
    Down,
    Left,
    Right,
}

pub fn moves_to_string(moves: &[Move]) -> String {
    use Move::*;
    moves
        .iter()
        .map(|m| match m {
            Up => 'U',
            Down => 'D',
            Left => 'L',
            Right => 'R',
        })
        .collect()
}

#[derive(Debug, PartialEq)]
struct OpenDoors {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl OpenDoors {
    pub fn new(salt: &str, moves: &[Move]) -> Self {
        let hash_str = format!("{salt}{}", moves_to_string(moves));
        let hash = format!("{:x}", md5::compute(&hash_str));
        let chars: Vec<_> = hash.chars().collect();
        OpenDoors {
            up: chars[0] >= 'b',
            down: chars[1] >= 'b',
            left: chars[2] >= 'b',
            right: chars[3] >= 'b',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn open_doors_new_test_1() {
        assert_eq!(
            OpenDoors::new("hijkl", &[]),
            OpenDoors {
                up: true,
                down: true,
                left: true,
                right: false
            }
        )
    }
}
