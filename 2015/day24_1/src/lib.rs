use itertools::Itertools;
use std::{
    cmp::Ordering,
    collections::BTreeSet,
    num::{NonZero, ParseIntError},
};

type Subset<T> = BTreeSet<T>;
type Partition<T> = BTreeSet<Subset<T>>;

pub fn parse_weights(lines: &[&str]) -> Result<Vec<u64>, ParseIntError> {
    lines.iter().map(|&line| line.parse()).collect()
}

#[derive(Debug)]
pub struct CannotPartition;

pub fn partitions(
    weights: BTreeSet<u64>,
    groups: NonZero<u64>,
) -> Result<BTreeSet<Partition<u64>>, CannotPartition> {
    if weights.is_empty() {
        return Err(CannotPartition);
    }
    if groups.get() == 1 as u64 {
        let partition = BTreeSet::from_iter([Subset::from_iter(weights)]);
        return Ok(BTreeSet::from_iter([partition]));
    }
    let sum = weights.iter().sum::<u64>();
    let group_sum = sum / groups;
    if sum % groups != 0 || *(weights.iter().max().unwrap()) > group_sum {
        return Err(CannotPartition);
    }
    let mut partitions_found = BTreeSet::new();
    for subset in weights.iter().cloned().powerset() {
        if subset.iter().map(|&i| i).sum::<u64>() != group_sum {
            continue;
        }
        let complement = weights
            .difference(&BTreeSet::from_iter(subset.iter().cloned()))
            .cloned()
            .collect();
        let sub_partitions = partitions(complement, NonZero::new(groups.get() - 1).unwrap());
        if sub_partitions.is_err() {
            continue;
        }
        let sub_partitions = sub_partitions.unwrap();
        for mut subpart in sub_partitions {
            let mut new_partition =
                BTreeSet::from_iter([Subset::from_iter(subset.iter().cloned())]);
            new_partition.append(&mut subpart);
            partitions_found.insert(new_partition);
        }
    }
    if partitions_found.is_empty() {
        return Err(CannotPartition);
    }
    Ok(partitions_found)
}

pub fn first_group(partition: &Partition<u64>) -> &Subset<u64> {
    let lowest_size = partition.iter().map(|group| group.len()).min().unwrap();
    partition
        .iter()
        .filter(|group| group.len() == lowest_size)
        .sorted_by(|a, b| {
            let diff = group_entanglement(a) as i64 - group_entanglement(b) as i64;
            if diff < 0 {
                return Ordering::Less;
            } else if diff > 0 {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        })
        .next()
        .unwrap()
}

pub fn group_entanglement(group: &Subset<u64>) -> u64 {
    group.iter().product()
}

pub fn entanglement(partition: &Partition<u64>) -> u64 {
    group_entanglement(first_group(partition))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_weights_test_1() {
        let parsed = parse_weights(&["1", "2", "3", "4"]);
        assert_eq!(Ok(vec![1, 2, 3, 4]), parsed);
    }

    #[test]
    fn parse_weights_test_2() {
        let parsed = parse_weights(&["1", "-2", "3", "4"]);
        assert!(parsed.is_err());
    }

    #[test]
    fn partitions_test_1() {
        let weights = BTreeSet::from_iter([1, 2, 3, 4, 5, 6]);
        let set_1_6 = Subset::from_iter([1, 6]);
        let set_2_5 = Subset::from_iter([2, 5]);
        let set_3_4 = Subset::from_iter([3, 4]);
        let partition = Partition::from_iter([set_1_6, set_2_5, set_3_4]);
        let correct = BTreeSet::from_iter([partition]);
        assert_eq!(
            partitions(weights, NonZero::new(3).unwrap()).unwrap(),
            correct
        );
    }

    #[test]
    fn partitions_test_2() {
        let weights = BTreeSet::from_iter([1, 3, 4, 5, 9]);
        assert!(partitions(weights, NonZero::new(2).unwrap()).is_err());
    }

    #[test]
    fn entanglement_test_1() {
        let set_1_6 = Subset::from_iter([1, 6]);
        let set_2_5 = Subset::from_iter([2, 5]);
        let set_3_4 = Subset::from_iter([3, 4]);
        let partition = Partition::from_iter([set_1_6, set_2_5, set_3_4]);
        assert_eq!(entanglement(&partition), 6);
    }
}
