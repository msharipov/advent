use std::num::NonZero;

#[derive(Debug, PartialEq)]
pub struct ElfCircle {
    present: Vec<usize>,
}

impl ElfCircle {
    pub fn new(count: NonZero<usize>) -> Self {
        let count: usize = count.into();
        ElfCircle {
            present: (1..=count).collect(),
        }
    }

    fn next_unskipped(&self, thief_index: usize) -> usize {
        (thief_index + 1) % self.present.len()
    }

    fn next_target(&self, thief_index: usize) -> usize {
        let len = self.present.len();
        let skip = len / 2;
        (thief_index + skip) % len
    }

    fn try_steal(&mut self, target_index: usize) -> Result<(), &str> {
        if target_index >= self.present.len() {
            return Err("target is invalid");
        }
        self.present.remove(target_index);
        if self.present.len() % 10000 == 0 {
            eprintln!("{} left", self.present.len());
        }
        Ok(())
    }
}

pub fn last_elf(count: NonZero<usize>) -> usize {
    let mut circle = ElfCircle::new(count);
    let mut current_thief = 0;
    loop {
        let target = circle.next_target(current_thief);
        if target == current_thief {
            return circle.present[current_thief];
        }
        if target < current_thief {
            current_thief -= 1;
        }
        circle
            .try_steal(target)
            .expect("target should be in bounds");
        current_thief = circle.next_unskipped(current_thief);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_unskipped_test_1() {
        let circle = ElfCircle {
            present: vec![2, 3],
        };
        assert_eq!(circle.next_unskipped(0), 1);
    }

    #[test]
    fn next_unskipped_test_2() {
        let circle = ElfCircle {
            present: vec![2, 3, 5],
        };
        assert_eq!(circle.next_unskipped(2), 0);
    }

    #[test]
    fn last_elf_index_test_1() {
        assert_eq!(last_elf(1.try_into().unwrap()), 1);
    }

    #[test]
    fn last_elf_index_test_2() {
        assert_eq!(last_elf(5.try_into().unwrap()), 2);
    }

    #[test]
    fn elf_circle_new_test_1() {
        assert_eq!(
            ElfCircle::new(3.try_into().unwrap()),
            ElfCircle {
                present: vec![1, 2, 3]
            }
        )
    }

    #[test]
    fn next_target_test_1() {
        let circle = ElfCircle::new(8.try_into().unwrap());
        assert_eq!(circle.next_target(2), 6);
    }

    #[test]
    fn next_target_test_2() {
        let circle = ElfCircle {
            present: vec![1, 2, 4, 8, 9],
        };
        assert_eq!(circle.next_target(2), 4);
    }

    #[test]
    fn try_steal_test_1() {
        let mut circle = ElfCircle {
            present: vec![1, 7, 8, 10, 11],
        };
        circle.try_steal(3).unwrap();
        assert_eq!(
            circle,
            ElfCircle {
                present: vec![1, 7, 8, 11],
            }
        );
        circle.try_steal(3).unwrap();
        assert_eq!(
            circle,
            ElfCircle {
                present: vec![1, 7, 8],
            }
        );
        assert!(circle.try_steal(3).is_err());
    }
}
