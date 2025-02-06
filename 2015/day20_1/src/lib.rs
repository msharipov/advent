use std::num::ParseIntError;

pub fn parse_input(lines: &[&str]) -> Result<i64, ParseIntError> {
    lines[0].parse::<i64>()
}

pub fn distinct_factors(product: u64) -> Vec<u64> {
    if product == 1 {
        return vec![1];
    }
    let mut factors = vec![];
    let mut other_factors = vec![];
    let root = (product as f64).sqrt().floor() as u64;
    for factor in 1..=root {
        if product % factor == 0 {
            factors.push(factor);
            let other = product / factor;
            if other != factor {
                other_factors.push(other);
            }
        }
    }
    other_factors.reverse();
    factors.append(&mut other_factors);
    factors
}

pub fn count_gifts(house: u64) -> u64 {
    distinct_factors(house).iter().sum::<u64>() * 10
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_input_test_1() {
        assert_eq!(parse_input(&["1435873"]), Ok(1435873));
    }

    #[test]
    fn parse_input_test_2() {
        assert!(parse_input(&["abracadabra"]).is_err());
    }

    #[test]
    fn distinct_factors_test_1() {
        assert_eq!(distinct_factors(1), vec![1]);
    }

    #[test]
    fn distinct_factors_test_2() {
        assert_eq!(distinct_factors(2), vec![1, 2]);
    }

    #[test]
    fn distinct_factors_test_3() {
        assert_eq!(distinct_factors(12), vec![1, 2, 3, 4, 6, 12]);
    }

    #[test]
    fn distinct_factors_test_4() {
        assert_eq!(distinct_factors(49), vec![1, 7, 49]);
    }

    #[test]
    fn count_gifts_test_1() {
        assert_eq!(count_gifts(1), 10);
    }

    #[test]
    fn count_gifts_test_2() {
        assert_eq!(count_gifts(6), 120);
    }
}
