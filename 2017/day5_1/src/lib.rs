use std::num::ParseIntError;

pub fn parse_jumps(lines: &[&str]) -> Result<Vec<i64>, ParseIntError> {
    lines.iter().map(|line| line.parse::<i64>()).collect()
}

pub fn jumps_to_exit(jumps: &[i64]) -> usize {
    let mut position: i64 = 0;
    let mut count = 0;
    let len = jumps.len() as i64;
    loop {
        if position >= len || position < 0 {
            return count;
        }
        position += jumps[position as usize] + 1;
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_jumps_test_1() {
        assert_eq!(
            parse_jumps(&["10", "-56", "0", "39"]),
            Ok(vec![10, -56, 0, 39])
        )
    }

    #[test]
    fn jumps_to_exit_test_1() {
        assert_eq!(jumps_to_exit(&[]), 0);
    }

    #[test]
    fn jumps_to_exit_test_2() {
        assert_eq!(jumps_to_exit(&[0, 0, 0, 0]), 4);
    }

    #[test]
    fn jumps_to_exit_test_3() {
        assert_eq!(jumps_to_exit(&[1, 3, 2, -6, 0, 4]), 3);
    }

    #[test]
    fn jumps_to_exit_test_4() {
        assert_eq!(jumps_to_exit(&[4, -3, 0, -3, 5, -4]), 5);
    }
}
