use std::num::ParseIntError;

pub fn parse_jumps(lines: &[&str]) -> Result<Vec<i64>, ParseIntError> {
    lines.iter().map(|line| line.parse::<i64>()).collect()
}

pub fn jumps_to_exit(jumps: &[i64]) -> usize {
    let mut jumps = jumps.to_owned();
    let mut position: i64 = 0;
    let mut count = 0;
    let len = jumps.len() as i64;
    loop {
        if position >= len || position < 0 {
            return count;
        }
        let old_position = position as usize;
        position += jumps[position as usize];
        jumps[old_position] += if jumps[old_position] >= 3 { -1 } else { 1 };
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
        assert_eq!(jumps_to_exit(&[0, 0, 0, 0]), 8);
    }

    #[test]
    fn jumps_to_exit_test_3() {
        assert_eq!(jumps_to_exit(&[1, 3, 2, -6, 0, -11]), 5);
    }

    #[test]
    fn jumps_to_exit_test_4() {
        assert_eq!(jumps_to_exit(&[3, -3, 0, -3, -4]), 7);
    }
}
