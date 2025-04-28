fn next_unskipped(skipped: &[bool], thief_index: usize) -> Option<usize> {
    if !matches!(skipped.get(thief_index), Some(false)) {
        return None;
    }
    let mut current_index = thief_index;
    let len = skipped.len();
    loop {
        current_index = (current_index + 1) % len;
        if !skipped[current_index] {
            return Some(current_index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_target_test_1() {
        assert_eq!(
            next_unskipped(&[true, true, false, false, true], 3),
            Some(2)
        );
    }

    #[test]
    fn get_target_test_2() {
        assert_eq!(next_unskipped(&[true, true, true, false, true], 3), Some(3));
    }

    #[test]
    fn get_target_test_3() {
        assert_eq!(next_unskipped(&[true, true, true, true, true], 3), None);
    }
}
