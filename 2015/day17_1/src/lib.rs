use itertools::Itertools;
use std::num::ParseIntError;

pub fn parse_containers(lines: &[&str]) -> Result<Vec<u64>, ParseIntError> {
    lines.iter().map(|line| line.parse::<u64>()).collect()
}

pub fn count_combinations(containers: &[u64], target: u64) -> u64 {
    containers
        .iter()
        .powerset()
        .map(|pset| {
            if pset.iter().copied().sum::<u64>() == target {
                1
            } else {
                0
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_combinations_test_1() {
        let containers = [1, 2, 3, 2, 4];
        assert_eq!(count_combinations(&containers, 4), 3);
    }

    #[test]
    fn count_combinations_test_2() {
        let containers = [20, 15, 10, 5, 5];
        assert_eq!(count_combinations(&containers, 25), 4);
    }}
