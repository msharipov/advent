use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug, PartialEq)]
pub struct Memory {
    banks: [u64; 16],
}

#[derive(Debug, PartialEq, Error)]
pub enum ParseBanksError {
    #[error("{0} banks found, should be 16")]
    WrongBanksNumber(usize),
    #[error("cannot parse the number of blocks")]
    WrongBlockCount(#[from] ParseIntError),
}

impl FromStr for Memory {
    type Err = ParseBanksError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Result<Vec<u64>, ParseIntError> =
            s.split('\t').map(|num| num.parse::<u64>()).collect();
        let numbers = numbers?;
        if numbers.len() != 16 {
            return Err(ParseBanksError::WrongBanksNumber(numbers.len()));
        }
        Ok(Memory {
            banks: numbers[..16].try_into().unwrap(),
        })
    }
}

impl Memory {
    fn bank_with_most_blocks(&self) -> usize {
        let max_blocks = self.banks.iter().max().unwrap();
        self.banks.iter().position(|x| x == max_blocks).unwrap()
    }

    pub fn redistribute(&self) -> Self {
        let mut current_bank = self.bank_with_most_blocks();
        let mut blocks_left = self.banks[current_bank];
        let mut new_banks = self.banks;
        new_banks[current_bank] = 0;
        while blocks_left > 0 {
            current_bank = (current_bank + 1) % 16;
            new_banks[current_bank] += 1;
            blocks_left -= 1;
        }
        Memory { banks: new_banks }
    }
}

#[cfg(test)]
mod tests {
    use std::num::IntErrorKind;

    use super::*;

    #[test]
    fn memory_from_str_test_1() {
        let mem = Memory {
            banks: [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16],
        };
        assert_eq!(
            "1\t2\t3\t4\t5\t6\t7\t8\t9\t10\t11\t12\t13\t14\t15\t16".parse(),
            Ok(mem)
        )
    }

    #[test]
    fn memory_from_str_test_2() {
        let res = "1\t2\t3\t4\t5\t6\t7\t8\t9\t-10\t11\t12\t13\t14\t15\t16".parse::<Memory>();
        assert!(if let Err(ParseBanksError::WrongBlockCount(err)) = res {
            matches!(err.kind(), IntErrorKind::InvalidDigit)
        } else {
            false
        })
    }

    #[test]
    fn memory_from_str_test_3() {
        let res = "1\t2\t3\t4\t5\t6\t7\t8\t9\t10\t11\t12\t13\t14\t15\t16\t17".parse::<Memory>();
        assert!(if let Err(ParseBanksError::WrongBanksNumber(num)) = res {
            num == 17
        } else {
            false
        })
    }

    #[test]
    fn memory_from_str_test_4() {
        let res = "1\t2\t3\t4\t12\t13\t14\t15\t16\t17".parse::<Memory>();
        assert!(if let Err(ParseBanksError::WrongBanksNumber(num)) = res {
            num == 10
        } else {
            false
        })
    }

    #[test]
    fn bank_with_most_blocks_test_1() {
        let mem = Memory { banks: [0; 16] };
        assert_eq!(mem.bank_with_most_blocks(), 0)
    }

    #[test]
    fn bank_with_most_blocks_test_2() {
        let mem = Memory {
            banks: [10, 1, 11, 5, 4, 15, 1, 4, 14, 7, 7, 15, 12, 3, 10, 2],
        };
        assert_eq!(mem.bank_with_most_blocks(), 5)
    }

    #[test]
    fn redistribute_test_1() {
        let mem = Memory { banks: [0; 16] };
        assert_eq!(mem.redistribute(), mem);
    }

    #[test]
    fn redistribute_test_2() {
        let mem = Memory { banks: [25; 16] };
        let correct = Memory {
            banks: [
                1, 27, 27, 27, 27, 27, 27, 27, 27, 27, 26, 26, 26, 26, 26, 26,
            ],
        };
        assert_eq!(mem.redistribute(), correct);
    }

    #[test]
    fn redistribute_test_3() {
        let mem = Memory {
            banks: [12, 9, 2, 2, 17, 10, 4, 22, 10, 1, 15, 16, 0, 22, 7, 11],
        };
        let correct = Memory {
            banks: [13, 10, 3, 3, 18, 11, 5, 1, 12, 3, 17, 18, 2, 24, 8, 12],
        };
        assert_eq!(mem.redistribute(), correct);
    }
}
