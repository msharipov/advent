use itertools::Itertools;
use std::{collections::HashMap, num::NonZeroU64};

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Offset {
    pub right: i64,
    pub up: i64,
}

pub fn offset(number: NonZeroU64) -> Offset {
    let number: u64 = number.into();
    if number == 1 {
        return Offset { right: 0, up: 0 };
    }
    let mut side_length: u64 = 3;
    while side_length.pow(2) < number {
        side_length += 2;
    }
    // Position along the outermost layer
    let outer_pos = number - (side_length - 2).pow(2) - 1;
    let side = outer_pos / (side_length - 1);
    // Index along the side
    let side_pos = (outer_pos % (side_length - 1)) as i64;
    let layer = (side_length / 2) as i64;
    match side {
        // Right
        0 => Offset {
            right: layer,
            up: side_pos - (layer - 1),
        },
        // Up
        1 => Offset {
            right: (layer - 1) - side_pos,
            up: layer,
        },
        // Left
        2 => Offset {
            right: -layer,
            up: (layer - 1) - side_pos,
        },
        // Down
        3 => Offset {
            right: side_pos - (layer - 1),
            up: -layer,
        },
        _ => panic!(),
    }
}

pub fn manhattan_distance(pt1: &Offset, pt2: &Offset) -> usize {
    (pt1.right.abs_diff(pt2.right) + pt1.up.abs_diff(pt2.up)) as usize
}

type Grid = HashMap<Offset, u64>;

pub fn first_value_larger_than(target: u64) -> u64 {
    let mut grid = Grid::new();
    grid.insert(Offset { right: 0, up: 0 }, 1);
    let mut count = 2;
    loop {
        let current_offset = offset(count.try_into().unwrap());
        let Offset { right, up } = current_offset;
        let neighbor_offsets = (-1..=1).cartesian_product(-1..=1).map(|pair| Offset {
            right: right + pair.0,
            up: up + pair.1,
        });
        let new_number = neighbor_offsets.flat_map(|offset| grid.get(&offset)).sum();
        if new_number > target {
            return new_number;
        }
        grid.insert(current_offset, new_number);
        count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn offset_test_1() {
        assert_eq!(offset(1.try_into().unwrap()), Offset { right: 0, up: 0 });
    }

    #[test]
    fn offset_test_2() {
        assert_eq!(offset(3.try_into().unwrap()), Offset { right: 1, up: 1 });
    }

    #[test]
    fn offset_test_3() {
        assert_eq!(offset(9.try_into().unwrap()), Offset { right: 1, up: -1 });
    }

    #[test]
    fn offset_test_4() {
        assert_eq!(offset(10.try_into().unwrap()), Offset { right: 2, up: -1 });
    }

    #[test]
    fn offset_test_5() {
        assert_eq!(offset(19.try_into().unwrap()), Offset { right: -2, up: 0 });
    }

    #[test]
    fn offset_test_6() {
        assert_eq!(offset(24.try_into().unwrap()), Offset { right: 1, up: -2 });
    }

    #[test]
    fn manhattan_distance_test_1() {
        assert_eq!(
            manhattan_distance(
                &Offset {
                    right: -75,
                    up: -75
                },
                &Offset {
                    right: -75,
                    up: -75
                }
            ),
            0
        );
    }

    #[test]
    fn manhattan_distance_test_2() {
        assert_eq!(
            manhattan_distance(
                &Offset { right: 35, up: 120 },
                &Offset {
                    right: -51,
                    up: -20
                }
            ),
            226
        );
    }

    #[test]
    fn first_value_larger_than_test_1() {
        assert_eq!(first_value_larger_than(1), 2);
    }

    #[test]
    fn first_value_larger_than_test_2() {
        assert_eq!(first_value_larger_than(4), 5);
    }

    #[test]
    fn first_value_larger_than_test_3() {
        assert_eq!(first_value_larger_than(78), 122);
    }

    #[test]
    fn first_value_larger_than_test_4() {
        assert_eq!(first_value_larger_than(800), 806);
    }
}
