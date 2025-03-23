use sscanf::sscanf;

type Sides = [u32; 3];

pub fn parse_sides(line: &str) -> Result<Sides, sscanf::Error> {
    let parsed = sscanf!(line, "{str:/[ ]+/}{u32}{str:/[ ]+/}{u32}{str:/[ ]+/}{u32}")?;
    Ok([parsed.1, parsed.3, parsed.5])
}

#[derive(Debug)]
pub enum BlockParseError {
    LineError(sscanf::Error),
    BlockError,
}

impl From<sscanf::Error> for BlockParseError {
    fn from(value: sscanf::Error) -> Self {
        Self::LineError(value)
    }
}

pub fn parse_all_triangles(lines: &[&str]) -> Result<Vec<Sides>, BlockParseError> {
    let mut sides = vec![];
    for block in lines.chunks(3) {
        if block.len() < 3 {
            return Err(BlockParseError::BlockError);
        }
        let first = parse_sides(block[0])?;
        let second = parse_sides(block[1])?;
        let third = parse_sides(block[2])?;
        sides.push([first[0], second[0], third[0]]);
        sides.push([first[1], second[1], third[1]]);
        sides.push([first[2], second[2], third[2]]);
    }
    Ok(sides)
}

pub fn valid_sides(sides: &Sides) -> bool {
    sides[0] < sides[1] + sides[2]
        && sides[1] < sides[0] + sides[2]
        && sides[2] < sides[0] + sides[1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_sides_test_1() {
        assert_eq!(parse_sides("  810  679   10").unwrap(), [810, 679, 10]);
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

    #[test]
    fn parse_all_triangles_test_1() {
        let lines = [
            " 101  201  301",
            " 102 202 302",
            " 103 203 303",
            " 401 501 601",
            " 402 502 602",
            " 403 503 603",
        ];
        let sides = parse_all_triangles(&lines);
        let correct = vec![
            [101, 102, 103],
            [201, 202, 203],
            [301, 302, 303],
            [401, 402, 403],
            [501, 502, 503],
            [601, 602, 603],
        ];
        assert_eq!(sides.unwrap(), correct);
    }

    #[test]
    fn parse_all_triangles_test_2() {
        let lines = [" 101  201  301", " 102 202 302"];
        let sides = parse_all_triangles(&lines);
        assert!(matches!(sides.unwrap_err(), BlockParseError::BlockError));
    }

    #[test]
    fn parse_all_triangles_test_3() {
        let lines = [" 101  201  301", " 102 202 302", " 103 303"];
        let sides = parse_all_triangles(&lines);
        assert!(matches!(sides.unwrap_err(), BlockParseError::LineError(_)));
    }
}
