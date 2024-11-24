pub fn count_repr_chars(s: &str) -> usize {
    format!("{:?}", s).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_repr_chars_test_1() {
        assert_eq!(count_repr_chars("\"ab\\\\\""), 12);
    }
}
