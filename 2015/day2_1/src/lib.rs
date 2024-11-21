pub struct Sides {
    length: u32,
    width: u32,
    height: u32,
}

impl Sides {
    pub fn smallest_side(&self) -> u32 {
        let mut sides = vec![self.length, self.width, self.height];
        sides.sort();
        sides[0] * sides[1]
    }

    pub fn needed_paper(&self) -> u32 {
        let area =
            2 * (self.length * self.width + self.length * self.height + self.width * self.height);
        area + self.smallest_side()
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
        assert_eq!(sides.smallest_side(), 180);
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
}
