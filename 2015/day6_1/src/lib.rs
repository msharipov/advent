use sscanf::sscanf;
use std::cmp::{max, min};

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

#[derive(Debug, PartialEq)]
struct Grid {
    values: Box<[bool; Grid::GRID_SIZE * Grid::GRID_SIZE]>,
}

impl Grid {
    const GRID_SIZE: usize = 1000;

    pub fn new() -> Self {
        Grid {
            values: Box::new([false; Grid::GRID_SIZE * Grid::GRID_SIZE]),
        }
    }

    fn apply(&mut self, act: GridAction) -> &mut Self {
        use GridAction::*;
        let rect = match &act {
            On(rect) => rect,
            Off(rect) => rect,
            Toggle(rect) => rect,
        };
        for x in rect.corner_1.x..rect.corner_2.x + 1 {
            for y in rect.corner_1.y..rect.corner_2.y + 1 {
                let index = Grid::GRID_SIZE * x + y;
                self.values[index] = match act {
                    On(_) => true,
                    Off(_) => false,
                    Toggle(_) => !self.values[index],
                }
            }
        }
        self
    }
}

impl Point {
    pub fn parse(s: &str) -> Result<Point, &'static str> {
        let parsed = sscanf!(s, "{usize},{usize}");
        if parsed.is_err() {
            return Err("can't parse coords");
        }
        let (x, y) = parsed.unwrap();
        if x >= Grid::GRID_SIZE || y >= Grid::GRID_SIZE {
            return Err("point out of bounds");
        }
        Ok(Point { x, y })
    }
}

impl Rect {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let corner_1 = Point {
            x: min(p1.x, p2.x),
            y: min(p1.y, p2.y),
        };
        let corner_2 = Point {
            x: max(p1.x, p2.x),
            y: max(p1.y, p2.y),
        };
        Rect { corner_1, corner_2 }
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
        assert_eq!(
            Ok(GridAction::Toggle(Rect {
                corner_1: Point { x: 567, y: 23 },
                corner_2: Point { x: 975, y: 800 }
            })),
            parse_action("toggle 975,23 through 567,800")
        )
    }

    #[test]
    fn parse_action_test_2() {
        assert_eq!(
            Ok(GridAction::On(Rect {
                corner_1: Point { x: 111, y: 504 },
                corner_2: Point { x: 111, y: 877 }
            })),
            parse_action("turn on 111,877 through 111,504")
        )
    }

    #[test]
    fn apply_action_test_1() {
        let a = Point { x: 567, y: 400 };
        let b = Point { x: 705, y: 600 };
        let c = Point { x: 705, y: 500 };
        let d = Point { x: 567, y: 501 };
        let fresh = Grid::new();
        let mut modified = Grid::new();
        modified
            .apply(GridAction::On(Rect::new(&a, &b)))
            .apply(GridAction::Off(Rect::new(&a, &c)))
            .apply(GridAction::Toggle(Rect::new(&d, &b)));
        assert_eq!(fresh, modified);
    }
}
