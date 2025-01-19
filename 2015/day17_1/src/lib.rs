use std::num::ParseIntError;

pub fn parse_containers(lines: &[&str]) -> Result<Vec<u64>, ParseIntError> {
    lines.iter().map(|line| line.parse::<u64>()).collect()
}
