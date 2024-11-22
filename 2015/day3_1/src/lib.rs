use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

fn parse_moves(s: &str) -> Vec<Move> {
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

fn count_distinct_houses(moves: &[Move]) -> usize {
    let mut visited = HashSet::new();
    let mut pos = (0i64, 0i64);
    visited.insert(pos);
    for m in moves {
        pos = next_pos(pos, m);
        visited.insert(pos);
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
        assert_eq!(count_distinct_houses(&parse_moves("^>v<>")), 4);
    }
}
