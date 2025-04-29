use std::ops::RangeInclusive;

type IpRange = RangeInclusive<u32>;

fn try_combine(range_1: &IpRange, range_2: &IpRange) -> Option<IpRange> {
    let range_2_follows_range_1 = range_1.end().saturating_add(1) == *range_2.start();
    if range_2.contains(range_1.end()) || range_2_follows_range_1 {
        let lower_bound = std::cmp::min(range_1.start(), range_2.start());
        let new_range = *lower_bound..=*range_2.end();
        return Some(new_range);
    }
    let range_1_follows_range_2 = range_2.end().saturating_add(1) == *range_1.start();
    if range_1.contains(range_2.end()) || range_1_follows_range_2 {
        let lower_bound = std::cmp::min(range_1.start(), range_2.start());
        let new_range = *lower_bound..=*range_1.end();
        return Some(new_range);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_combine_test_1() {
        assert_eq!(try_combine(&(0..=5), &(7..=8)), None);
        assert_eq!(try_combine(&(11..=14), &(3..=6)), None);
    }

    #[test]
    fn try_combine_test_2() {
        assert_eq!(try_combine(&(0..=5), &(6..=8)), Some(0..=8));
        assert_eq!(try_combine(&(11..=14), &(3..=10)), Some(3..=14));
    }

    #[test]
    fn try_combine_test_3() {
        assert_eq!(try_combine(&(0..=5), &(2..=8)), Some(0..=8));
        assert_eq!(try_combine(&(8..=14), &(3..=10)), Some(3..=14));
    }

    #[test]
    fn try_combine_test_4() {
        assert_eq!(try_combine(&(2..=17), &(6..=8)), Some(2..=17));
        assert_eq!(try_combine(&(11..=13), &(3..=14)), Some(3..=14));
    }
}
