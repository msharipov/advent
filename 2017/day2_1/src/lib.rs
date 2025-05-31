fn max_minus_min(list: &[i64]) -> Option<i64> {
    Some(list.iter().max()? - list.iter().min()?)
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
}
