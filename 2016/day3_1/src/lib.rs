use sscanf::sscanf;

pub fn parse_sides(line: &str) -> Result<[u32; 3], sscanf::Error> {
    let parsed = sscanf!(line, "{str:/[ ]+/}{u32}{str:/[ ]+/}{u32}{str:/[ ]+/}{u32}")?;
    Ok([parsed.1, parsed.3, parsed.5])
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
}
