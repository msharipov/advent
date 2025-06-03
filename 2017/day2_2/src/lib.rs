use itertools::Itertools;
use std::num::ParseIntError;

fn max_minus_min(list: &[i64]) -> Option<i64> {
    Some(list.iter().max()? - list.iter().min()?)
}

pub fn parse_lines(lines: &[&str]) -> Result<Vec<Vec<i64>>, ParseIntError> {
    let mut vecs = vec![];
    for line in lines {
        vecs.push(
            line.split([' ', '\t'])
                .map(|num| num.parse())
                .collect::<Result<Vec<_>, ParseIntError>>()?,
        );
    }
    Ok(vecs)
}

pub fn compute_checksum(numbers: &[Vec<i64>]) -> Option<i64> {
    numbers.iter().map(|vec| max_minus_min(vec)).sum()
}

fn evenly_divisible_pair(list: &[i64]) -> Option<(i64, i64)> {
    let divisible_pair = list
        .iter()
        .permutations(2)
        .find(|pair| pair[0] % pair[1] == 0)?;
    Some((*divisible_pair[0], *divisible_pair[1]))
}

pub fn compute_checksum_2(numbers: &[Vec<i64>]) -> Option<i64> {
    numbers
        .iter()
        .map(|vec| {
            let pair = evenly_divisible_pair(vec)?;
            Some(pair.0 / pair.1)
        })
        .sum()
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
        let lines = ["12 34 56 78", "1 2 3\t4567 890"];
        let correct = vec![vec![12, 34, 56, 78], vec![1, 2, 3, 4567, 890]];
        assert_eq!(parse_lines(&lines), Ok(correct));
    }

    #[test]
    fn parse_lines_test_2() {
        let lines = ["12 34 56 78", "1 2 c 4567 890"];
        assert!(parse_lines(&lines).is_err());
    }

    #[test]
    fn compute_checksum_test_1() {
        let numbers = [
            vec![-43, 56, -123, 74, 153, 90, 25],
            vec![56, 14, -45, 100, 40, 431, -59],
            vec![504],
        ];
        assert_eq!(compute_checksum(&numbers), Some(276 + 490));
    }

    #[test]
    fn compute_checksum_test_2() {
        let numbers = [
            vec![-43, 56, -123, 74, 153, 90, 25],
            vec![],
            vec![56, 14, -45, 100, 40, 431, -59],
            vec![504],
        ];
        assert_eq!(compute_checksum(&numbers), None);
    }

    #[test]
    fn evenly_divisible_pair_test_1() {
        assert_eq!(
            evenly_divisible_pair(&[13, 22, 10, 15, 9, 17, 27]),
            Some((27, 9))
        );
    }

    #[test]
    fn evenly_divisible_pair_test_2() {
        assert_eq!(evenly_divisible_pair(&[13, 22, 10, 15, 4, 17, 27]), None);
    }

    #[test]
    fn compute_checksum_2_test_1() {
        let numbers = [vec![11, 5, 64, 12, 20, 19, 28]];
        assert_eq!(compute_checksum_2(&numbers), Some(4));
    }

    #[test]
    fn compute_checksum_2_test_2() {
        let numbers = [vec![11, 5, 64, 12, 20, 19, 28], vec![103, 10, 24, 8, 89]];
        assert_eq!(compute_checksum_2(&numbers), Some(7));
    }

    #[test]
    fn compute_checksum_3_test_2() {
        let numbers = [
            vec![11, 5, 64, 12, 20, 19, 28],
            vec![103, 10, 24, 8, 89],
            vec![45],
        ];
        assert_eq!(compute_checksum_2(&numbers), None);
    }
}
