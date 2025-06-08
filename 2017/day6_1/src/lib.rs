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
}
