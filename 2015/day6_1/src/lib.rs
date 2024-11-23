use sscanf::sscanf;
use std::error::Error;

#[derive(Debug, PartialEq, Clone)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug, PartialEq)]
struct Rect {
    corner_1: Point,
    corner_2: Point,
}

#[derive(Debug, PartialEq)]
enum GridAction {
    On(Rect),
    Off(Rect),
    Toggle(Rect),
}

const GRID_SIZE: usize = 1000;

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

impl Rect {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let mut first: &Point = p1;
        let mut second: &Point = p2;
        if p2.x < p1.x || (p2.x == p1.x && p2.y < p1.y) {
            first = p2;
            second = p1;
        }
        Rect {
            corner_1: (*first).clone(),
            corner_2: (*second).clone(),
        }
    }
}

fn parse_action(s: &str) -> Result<GridAction, &'static str> {
    let tokens: Vec<_> = s.split(' ').collect();
    if tokens.len() == 5 && tokens[0] == "turn" {
        let corner_1 = Point::parse(tokens[2])?;
        let corner_2 = Point::parse(tokens[4])?;
        let rect = Rect::new(&corner_1, &corner_2);
        match tokens[1] {
            "on" => return Ok(GridAction::On(rect)),
            "off" => return Ok(GridAction::Off(rect)),
            _ => return Err("invalid token after turn"),
        }
    } else if tokens.len() == 4 && tokens[0] == "toggle" {
        let corner_1 = Point::parse(tokens[1])?;
        let corner_2 = Point::parse(tokens[3])?;
        let rect = Rect::new(&corner_1, &corner_2);
        return Ok(GridAction::Toggle(rect));
    }
    Err("invalid command")
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

    #[test]
    fn parse_action_test_1() {
        assert_eq!(Ok(GridAction::Toggle(Rect{
            corner_1: Point {
                x: 567,
                y: 800,
            },
            corner_2: Point {
                x: 975,
                y: 23,
            }
        })), parse_action("toggle 975,23 through 567,800"))
    }

    #[test]
    fn parse_action_test_2() {
        assert_eq!(Ok(GridAction::On(Rect{
            corner_1: Point {
                x: 111,
                y: 504,
            },
            corner_2: Point {
                x: 111,
                y: 877,
            }
        })), parse_action("turn on 111,877 through 111,504"))
    }
}
