#[derive(Debug, PartialEq)]
pub enum Move {
    Up,
    Right,
    Down,
    Left,
}

fn parse_moves(s: &str) -> Vec<Move> {
    s.chars().filter_map(|c| match c {
        '^' => Some(Move::Up),
        '>' => Some(Move::Right),
        'v' => Some(Move::Down),
        '<' => Some(Move::Left),
        _ => None,
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use Move::*;

    #[test]
    fn parse_moves_test_1() {
        assert_eq!(parse_moves("^vb<>8"), vec![Up, Down, Left, Right]);
    }
}
