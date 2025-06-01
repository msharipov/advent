use std::num::ParseIntError;

fn max_minus_min(list: &[i64]) -> Option<i64> {
    Some(list.iter().max()? - list.iter().min()?)
}

fn parse_lines(lines: &[&str]) -> Result<Vec<Vec<i64>>, ParseIntError> {
    let mut vecs = vec![];
    for line in lines {
        vecs.push(
            line.split(' ')
                .map(|num| num.parse())
                .collect::<Result<Vec<_>, ParseIntError>>()?,
        );
    }
    Ok(vecs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_minus_min_test_1() {
        assert_eq!(max_minus_min(&[2, -5, 12, 9, -3, 0, 14]), Some(19))
    }

    #[test]
    fn max_minus_min_test_2() {
        assert_eq!(max_minus_min(&[11]), Some(0))
    }

    #[test]
    fn max_minus_min_test_3() {
        assert_eq!(max_minus_min(&[]), None)
    }

    #[test]
    fn parse_lines_test_1() {
        let lines = ["12 34 56 78", "1 2 3 4567 890"];
        let correct = vec![vec![12, 34, 56, 78], vec![1, 2, 3, 4567, 890]];
        assert_eq!(parse_lines(&lines), Ok(correct));
    }

    #[test]
    fn parse_lines_test_2() {
        let lines = ["12 34 56 78", "1 2 c 4567 890"];
        assert!(parse_lines(&lines).is_err());
    }
}
