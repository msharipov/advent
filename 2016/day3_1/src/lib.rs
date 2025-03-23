use sscanf::sscanf;

pub fn parse_sides(line: &str) -> Result<[u32; 3], sscanf::Error> {
    let parsed = sscanf!(line, "{str:/[ ]+/}{u32}{str:/[ ]+/}{u32}{str:/[ ]+/}{u32}")?;
    Ok([parsed.1, parsed.3, parsed.5])
}

pub fn valid_sides(sides: &[u32; 3]) -> bool {
    sides[0] < sides[1] + sides[2]
        && sides[1] < sides[0] + sides[2]
        && sides[2] < sides[0] + sides[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sides_test_1() {
        assert_eq!(parse_sides(" 123 456  78").unwrap(), [123, 456, 78]);
    }

    #[test]
    fn parse_sides_test_2() {
        assert!(parse_sides(" 123 4545").is_err());
    }

    #[test]
    fn valid_sides_test_1() {
        assert!(valid_sides(&[3, 4, 5]));
    }

    #[test]
    fn valid_sides_test_2() {
        assert!(!valid_sides(&[2, 2, 4]));
    }
}
