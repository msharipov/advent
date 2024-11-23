use sscanf::sscanf;
use std::error::Error;

#[derive(Debug, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

struct Rect {
    corner_1: Point,
    corner_2: Point,
}

enum GridAction {
    On(Rect),
    Off(Rect),
    Toggle(Rect),
}

const GRID_SIZE: usize = 1000;

struct ActionError;

impl Point {
    pub fn parse(s: &str) -> Result<Point, &'static str> {
        let parsed = sscanf!(s, "{usize},{usize}");
        if let Err(_) = parsed {
            return Err("can't parse coords");
        }
        let (x, y) = parsed.unwrap();
        if x >= GRID_SIZE || y >= GRID_SIZE {
            return Err("point out of bounds");
        }
        Ok(Point { x, y })
    }
}

pub fn parse_action(s: &str) -> Result<GridAction, ActionError> {
    let tokens: Vec<_> = s.split(' ').collect();
    match tokens.len() {
        4 | 5 => {}
        _ => return Err(ActionError),
    };
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_parse_test_1() {
        assert_eq!(Ok(Point { x: 567, y: 19 }), Point::parse("567,19"));
    }

    #[test]
    fn point_parse_test_2() {
        assert_eq!(Err("can't parse coords"), Point::parse("hello"));
    }

    #[test]
    fn point_parse_test_3() {
        assert_eq!(Err("point out of bounds"), Point::parse("1354,90"));
    }
}
