use std::num::NonZero;

#[derive(Debug, PartialEq)]
pub struct ElfCircle {
    skipped: Vec<bool>,
    present: usize,
}

impl ElfCircle {
    pub fn new(count: NonZero<usize>) -> Self {
        let count: usize = count.into();
        ElfCircle {
            skipped: vec![false; count],
            present: count,
        }
    }

    fn next_unskipped(&self, thief_index: usize) -> Option<usize> {
        if !matches!(self.skipped.get(thief_index), Some(false)) {
            return None;
        }
        let mut current_index = thief_index;
        let len = self.skipped.len();
        loop {
            current_index = (current_index + 1) % len;
            if !self.skipped[current_index] {
                return Some(current_index);
            }
        }
    }
}

pub fn last_elf_index(count: NonZero<usize>) -> usize {
    let mut circle = ElfCircle::new(count);
    let mut current_thief = 0;
    loop {
        let target = circle
            .next_unskipped(current_thief)
            .expect("there should always be at least one unskipped elf");
        if target == current_thief {
            return current_thief;
        }
        circle.skipped[target] = true;
        current_thief = circle
            .next_unskipped(current_thief)
            .expect("there should always be at least one unskipped elf");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_unskipped_test_1() {
        let circle = ElfCircle {
            skipped: vec![true, true, false, false, true],
            present: 2,
        };
        assert_eq!(circle.next_unskipped(3), Some(2));
    }

    #[test]
    fn next_unskipped_test_2() {
        let circle = ElfCircle {
            skipped: vec![true, true, true, false, true],
            present: 1,
        };
        assert_eq!(circle.next_unskipped(3), Some(3));
    }

    #[test]
    fn next_unskipped_test_3() {
        let circle = ElfCircle {
            skipped: vec![true, true, true, true, true],
            present: 0,
        };
        assert_eq!(circle.next_unskipped(3), None);
    }

    #[test]
    fn last_elf_index_test_1() {
        assert_eq!(last_elf_index(1.try_into().unwrap()), 0);
    }

    #[test]
    fn last_elf_index_test_2() {
        assert_eq!(last_elf_index(5.try_into().unwrap()), 2);
    }

    #[test]
    fn elf_circle_new_test_1() {
        assert_eq!(
            ElfCircle::new(3.try_into().unwrap()),
            ElfCircle {
                skipped: vec![false; 3],
                present: 3
            }
        )
    }
}
