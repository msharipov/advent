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
    let root = (product as f64).sqrt().ceil() as u64;
    for factor in 1..root {
        if product % factor == 0 {
            factors.push(factor);
            other_factors.push(product / factor);
        }
    }
    other_factors.reverse();
    factors.append(&mut other_factors);
    factors
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
}
