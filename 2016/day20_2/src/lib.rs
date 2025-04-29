use std::{collections::HashSet, ops::RangeInclusive};

use sscanf::sscanf;

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

pub fn parse_ranges(lines: &[&str]) -> Result<HashSet<IpRange>, sscanf::Error> {
    let mut ranges: HashSet<IpRange> = HashSet::new();
    for line in lines {
        let (lower, upper) = sscanf!(line, "{u32}-{u32}")?;
        let mut range = lower..=upper;
        let mut absorbed = vec![];
        for other_range in ranges.iter() {
            if let Some(combined) = try_combine(&range, other_range) {
                absorbed.push(other_range.clone());
                range = combined;
            }
        }
        for old_range in absorbed {
            ranges.remove(&old_range);
        }
        ranges.insert(range);
    }
    Ok(ranges)
}

pub fn lowest_allowed_ip(ranges: &HashSet<IpRange>) -> Option<u32> {
    let lowest_range = ranges.iter().min_by_key(|range| range.start())?;
    if lowest_range.end() == &u32::MAX {
        None
    } else {
        Some(*lowest_range.end() + 1)
    }
}

pub fn count_allowed_ips(ranges: &HashSet<IpRange>) -> usize {
    let mut allowed = u32::MAX as usize + 1;
    for range in ranges {
        let range_size = *range.end() as usize - *range.start() as usize + 1;
        allowed -= range_size;
    }
    allowed
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

    #[test]
    fn parse_ranges_test_1() {
        let lines = ["100-145", "79-99", "11-36", "20-25", "52-62", "60-71"];
        let correct = HashSet::from_iter([11..=36, 52..=71, 79..=145]);
        assert_eq!(parse_ranges(&lines).unwrap(), correct);
    }

    #[test]
    fn lowest_allowed_ip_test_1() {
        let ranges = HashSet::from_iter([0..=14, 17..=28, 32..=40]);
        assert_eq!(lowest_allowed_ip(&ranges), Some(15));
    }

    #[test]
    fn lowest_allowed_ip_test_2() {
        let ranges = HashSet::from_iter([0..=u32::MAX]);
        assert_eq!(lowest_allowed_ip(&ranges), None);
    }

    #[test]
    fn count_allowed_ips_test_1() {
        let ranges = HashSet::from_iter([0..=56, 60..=104, 115..=u32::MAX]);
        assert_eq!(count_allowed_ips(&ranges), 13);
    }

    #[test]
    fn count_allowed_ips_test_2() {
        let ranges = HashSet::from_iter([0..=u32::MAX]);
        assert_eq!(count_allowed_ips(&ranges), 0);
    }
}
