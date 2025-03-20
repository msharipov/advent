use itertools::Itertools;
use std::{
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

pub fn partition_into_thirds(
    weights: BTreeSet<u64>,
) -> Result<BTreeSet<Partition<u64>>, CannotPartition> {
    let mut equal_groups = groups_by_size(weights, NonZero::new(3).unwrap())?;
    let mut partitions_found = BTreeSet::new();
    for first_group in equal_groups.clone().iter() {
        let available_groups = equal_groups
            .iter()
            .filter(|group| group.is_disjoint(first_group))
            .collect_vec();
        for &second_group in &available_groups {
            let last_groups = available_groups
                .iter()
                .filter(|group| group.is_disjoint(second_group))
                .collect_vec();
            for &last_group in last_groups {
                let mut new_partition = Partition::new();
                new_partition.insert(first_group.clone());
                new_partition.insert(second_group.clone());
                new_partition.insert(last_group.clone());
                partitions_found.insert(new_partition);
            }
        }
        equal_groups.remove(first_group);
    }
    if partitions_found.is_empty() {
        return Err(CannotPartition);
    }
    Ok(partitions_found)
}

pub fn groups_by_size(
    weights: BTreeSet<u64>,
    groups: NonZero<u64>,
) -> Result<BTreeSet<Subset<u64>>, CannotPartition> {
    if weights.is_empty() {
        return Err(CannotPartition);
    }
    if groups.get() == 1_u64 {
        return Ok(Subset::from_iter([weights]));
    }
    let sum = weights.iter().sum::<u64>();
    let group_sum = sum / groups;
    if sum % groups != 0 || *(weights.iter().max().unwrap()) > group_sum {
        return Err(CannotPartition);
    }
    let groups = weights
        .iter()
        .powerset()
        .filter(|group| group.iter().cloned().sum::<u64>() == group_sum)
        .map(|vec| BTreeSet::from_iter(vec.iter().map(|x| **x)));
    Ok(BTreeSet::from_iter(groups))
}

pub fn first_group(partition: &Partition<u64>) -> &Subset<u64> {
    let lowest_size = partition.iter().map(|group| group.len()).min().unwrap();
    partition
        .iter()
        .filter(|group| group.len() == lowest_size)
        .sorted_by(|a, b| {
            let diff = group_entanglement(a) as i64 - group_entanglement(b) as i64;
            diff.cmp(&0)
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
    fn partition_into_thirds_test_1() {
        let weights = BTreeSet::from_iter([1, 2, 3, 4, 5, 6]);
        let set_1_6 = Subset::from_iter([1, 6]);
        let set_2_5 = Subset::from_iter([2, 5]);
        let set_3_4 = Subset::from_iter([3, 4]);
        let partition = Partition::from_iter([set_1_6, set_2_5, set_3_4]);
        let correct = BTreeSet::from_iter([partition]);
        assert_eq!(partition_into_thirds(weights).unwrap(), correct);
    }

    #[test]
    fn partition_into_thirds_test_2() {
        let weights = BTreeSet::from_iter([1, 3, 4, 5, 9]);
        assert!(partition_into_thirds(weights).is_err());
    }

    #[test]
    fn entanglement_test_1() {
        let set_1_6 = Subset::from_iter([1, 6]);
        let set_2_5 = Subset::from_iter([2, 5]);
        let set_3_4 = Subset::from_iter([3, 4]);
        let partition = Partition::from_iter([set_1_6, set_2_5, set_3_4]);
        assert_eq!(entanglement(&partition), 6);
    }

    #[test]
    fn groups_by_size_test_1() {
        let weights = BTreeSet::from_iter([1, 2, 3, 4, 5, 6]);
        let set_1_6 = Subset::from_iter([1, 6]);
        let set_2_5 = Subset::from_iter([2, 5]);
        let set_3_4 = Subset::from_iter([3, 4]);
        let set_1_2_4 = Subset::from_iter([1, 2, 4]);
        let correct = BTreeSet::from_iter([set_1_6, set_2_5, set_3_4, set_1_2_4]);
        let grouped = groups_by_size(weights, NonZero::new(3).unwrap());
        assert_eq!(correct, grouped.unwrap());
    }
}
