#[derive(Debug, PartialEq, Clone)]
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

type Path = (u8, u8, Vec<Move>);

pub fn shortest_path(salt: &str) -> Option<Vec<Move>> {
    let mut horizon: Vec<Path> = vec![(0, 0, vec![])];
    loop {
        if horizon.is_empty() {
            return None;
        }
        let mut new_horizon = vec![];
        for (row, col, moves) in horizon {
            if row == 3 && col == 3 {
                return Some(moves);
            }
            let open_doors = OpenDoors::new(salt, &moves);
            if row > 0 && open_doors.up {
                let mut new_moves = moves.clone();
                new_moves.push(Move::Up);
                new_horizon.push((row - 1, col, new_moves));
            }
            if row < 3 && open_doors.down {
                let mut new_moves = moves.clone();
                new_moves.push(Move::Down);
                new_horizon.push((row + 1, col, new_moves));
            }
            if col > 0 && open_doors.left {
                let mut new_moves = moves.clone();
                new_moves.push(Move::Left);
                new_horizon.push((row, col - 1, new_moves));
            }
            if col < 3 && open_doors.right {
                let mut new_moves = moves.clone();
                new_moves.push(Move::Right);
                new_horizon.push((row, col + 1, new_moves));
            }
        }
        horizon = new_horizon;
    }
}

pub fn longest_path(salt: &str) -> usize {
    let mut horizon: Vec<Path> = vec![(0, 0, vec![])];
    let mut last_solution = vec![];
    loop {
        if horizon.is_empty() {
            return last_solution.len();
        }
        let mut new_horizon = vec![];
        for (row, col, moves) in horizon {
            if row == 3 && col == 3 {
                last_solution = moves.clone();
                continue;
            }
            let open_doors = OpenDoors::new(salt, &moves);
            if row > 0 && open_doors.up {
                let mut new_moves = moves.clone();
                new_moves.push(Move::Up);
                new_horizon.push((row - 1, col, new_moves));
            }
            if row < 3 && open_doors.down {
                let mut new_moves = moves.clone();
                new_moves.push(Move::Down);
                new_horizon.push((row + 1, col, new_moves));
            }
            if col > 0 && open_doors.left {
                let mut new_moves = moves.clone();
                new_moves.push(Move::Left);
                new_horizon.push((row, col - 1, new_moves));
            }
            if col < 3 && open_doors.right {
                let mut new_moves = moves.clone();
                new_moves.push(Move::Right);
                new_horizon.push((row, col + 1, new_moves));
            }
        }
        horizon = new_horizon;
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

    #[test]
    fn shortest_path_test_1() {
        assert_eq!(
            moves_to_string(&shortest_path("ihgpwlah").unwrap()),
            "DDRRRD"
        );
    }

    #[test]
    fn shortest_path_test_2() {
        assert_eq!(
            moves_to_string(&shortest_path("kglvqrro").unwrap()),
            "DDUDRLRRUDRD"
        );
    }

    #[test]
    fn longest_path_test_1() {
        assert_eq!(longest_path("ihgpwlah"), 370);
    }
}
