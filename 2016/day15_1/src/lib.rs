use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct Disc {
    positions: u64,
    start: u64,
}

impl Disc {
    pub fn new(positions: u64, start: u64) -> Self {
        Disc { positions, start }
    }
}

impl FromStr for Disc {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (_, positions, start) = sscanf!(
            s,
            "Disc #{u64} has {u64} positions; at time=0, it is at position {u64}."
        )?;
        Ok(Disc::new(positions, start))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disc_from_str_test_1() {
        assert_eq!(
            "Disc #123 has 55 positions; at time=0, it is at position 20."
                .parse::<Disc>()
                .unwrap(),
            Disc::new(55, 20)
        );
    }
}
