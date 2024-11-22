use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

pub fn parse_moves(s: &str) -> Vec<Move> {
    s.chars()
        .filter_map(|c| match c {
            '^' => Some(Move::Up),
            '>' => Some(Move::Right),
            'v' => Some(Move::Down),
            '<' => Some(Move::Left),
            _ => None,
        })
        .collect()
}

fn next_pos(current: (i64, i64), dir: &Move) -> (i64, i64) {
    let (x, y) = current;
    match dir {
        Move::Up => (x, y + 1),
        Move::Down => (x, y - 1),
        Move::Left => (x - 1, y),
        Move::Right => (x + 1, y),
    }
}

pub fn count_distinct_houses(moves: &[Move]) -> usize {
    let mut visited = HashSet::new();
    let mut santa_pos = (0i64, 0i64);
    let mut robot_pos = (0i64, 0i64);
    let mut santa_moves = true;
    visited.insert(santa_pos);
    for m in moves {
        match santa_moves {
            true => {
                santa_pos = next_pos(santa_pos, m);
                visited.insert(santa_pos);
            }
            false => {
                robot_pos = next_pos(robot_pos, m);
                visited.insert(robot_pos);
            }
        }
        santa_moves = !santa_moves;
    }
    visited.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Move::*;

    #[test]
    fn parse_moves_test_1() {
        assert_eq!(parse_moves("^vb<>8"), vec![Up, Down, Left, Right]);
    }

    #[test]
    fn count_distinct_houses_test_1() {
        assert_eq!(count_distinct_houses(&parse_moves("^>v<>")), 3);
    }
}
