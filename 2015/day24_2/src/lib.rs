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

pub fn lowest_entanglement(weights: BTreeSet<u64>) -> Result<u64, CannotPartition> {
    let mut equal_groups = groups_by_size(weights, NonZero::new(4).unwrap())?;
    let mut best = u64::MAX;
    for first_group in equal_groups.clone().iter() {
        let second_groups = equal_groups
            .iter()
            .filter(|group| group.is_disjoint(&first_group))
            .collect_vec();
        for &second_group in &second_groups {
            let third_groups = second_groups
                .iter()
                .filter(|group| group.is_disjoint(&second_group))
                .collect_vec();
            for &&third_group in &third_groups {
                let fourth_groups = third_groups
                    .iter()
                    .filter(|group| group.is_disjoint(&third_group))
                    .collect_vec();
                for &&fourth_group in fourth_groups {
                    let mut new_partition = Partition::new();
                    new_partition.insert(first_group.clone());
                    new_partition.insert(second_group.clone());
                    new_partition.insert(third_group.clone());
                    new_partition.insert(fourth_group.clone());
                    let ent = entanglement(&new_partition);
                    if ent < best {
                        best = ent;
                    }
                }
            }
        }
        equal_groups.remove(first_group);
    }
    if best == u64::MAX {
        return Err(CannotPartition);
    }
    Ok(best)
}

pub fn groups_by_size(
    weights: BTreeSet<u64>,
    groups: NonZero<u64>,
) -> Result<BTreeSet<Subset<u64>>, CannotPartition> {
    if weights.is_empty() {
        return Err(CannotPartition);
    }
    if groups.get() == 1 as u64 {
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
        .filter(|group| group.iter().map(|x| *x).sum::<u64>() == group_sum)
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
    fn lowest_entanglement_test_1() {
        let weights = BTreeSet::from_iter([1, 2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(lowest_entanglement(weights).unwrap(), 8);
    }

    #[test]
    fn partition_into_thirds_test_2() {
        let weights = BTreeSet::from_iter([1, 3, 4, 5, 9]);
        assert!(lowest_entanglement(weights).is_err());
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
