use std::num::NonZeroU64;

#[derive(Debug, PartialEq)]
pub struct Offset {
    right: i64,
    up: i64,
}

fn offset(number: NonZeroU64) -> Offset {
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
}
