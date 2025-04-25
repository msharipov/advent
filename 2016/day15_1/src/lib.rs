use num::integer;
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

fn discs_aligned(discs: &[Disc], time: u64) -> bool {
    discs
        .iter()
        .enumerate()
        .all(|(i, disc)| (disc.start + 1 + i as u64 + time) % disc.positions == 0)
}

pub fn first_capsule_time(discs: &[Disc]) -> Option<u64> {
    let period = discs
        .iter()
        .fold(1, |acc, disc| integer::lcm(acc, disc.positions));
    (0..period).find(|&t| discs_aligned(discs, t))
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

    #[test]
    fn first_capsule_time_test_1() {
        let discs = [Disc::new(5, 2), Disc::new(3, 0), Disc::new(4, 1)];
        assert_eq!(first_capsule_time(&discs), Some(52));
    }

    #[test]
    fn first_capsule_time_test_2() {
        let discs = [Disc::new(3, 1), Disc::new(3, 2), Disc::new(7, 4)];
        assert_eq!(first_capsule_time(&discs), None);
    }
}
