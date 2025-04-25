fn next_iter(initial: &str) -> String {
    let b: String = initial
        .chars()
        .rev()
        .map(|c| if c == '0' { '1' } else { '0' })
        .collect();
    format!("{initial}0{b}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_iter_test_1() {
        assert_eq!(next_iter("1010"), "101001010");
    }
}
