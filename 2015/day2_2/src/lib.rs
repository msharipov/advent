use std::{num::ParseIntError, str::FromStr};

pub struct Sides {
    length: u32,
    width: u32,
    height: u32,
}

impl Sides {
    pub fn smallest_side(&self) -> (u32, u32) {
        let mut sides = [self.length, self.width, self.height];
        sides.sort();
        (sides[0], sides[1])
    }

    pub fn needed_paper(&self) -> u32 {
        let area =
            2 * (self.length * self.width + self.length * self.height + self.width * self.height);
        let smallest = self.smallest_side();
        area + smallest.0 * smallest.1
    }

    pub fn needed_ribbon(&self) -> u32 {
        let volume = self.length * self.width * self.height;
        let smallest = self.smallest_side();
        volume + 2 * (smallest.0 + smallest.1)
    }
}

#[derive(Debug)]
pub struct SidesParseErr;

impl FromStr for Sides {
    type Err = SidesParseErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sides = match s
            .split("x")
            .map(|s| s.parse())
            .collect::<Result<Vec<_>, ParseIntError>>()
        {
            Ok(s) => Ok(s),
            Err(_) => Err(SidesParseErr),
        }?;
        if sides.len() != 3 {
            return Err(SidesParseErr);
        }
        Ok(Sides {
            length: sides[0],
            width: sides[1],
            height: sides[2],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_side_test_1() {
        let sides = Sides {
            length: 12,
            width: 17,
            height: 15,
        };
        assert_eq!(sides.smallest_side(), (12, 15));
    }

    #[test]
    fn needed_paper_test_1() {
        let sides = Sides {
            length: 12,
            width: 17,
            height: 15,
        };
        assert_eq!(sides.needed_paper(), 1458);
    }

    #[test]
    fn sides_from_str_test_1() {
        let sides: Sides = "12x17x15".parse().unwrap();
        assert_eq!(sides.needed_paper(), 1458);
    }

    #[test]
    fn needed_ribbon_test_1() {
        let sides: Sides = "8x3x5".parse().unwrap();
        assert_eq!(sides.needed_ribbon(), 136);
    }
}
