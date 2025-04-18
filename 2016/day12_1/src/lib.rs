use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Register {
    A,
    B,
    C,
    D,
}

impl FromStr for Register {
    type Err = sscanf::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "a" => Ok(Register::A),
            "b" => Ok(Register::B),
            "c" => Ok(Register::C),
            "d" => Ok(Register::D),
            _ => Err(sscanf::Error::MatchFailed),
        }
    }
}

pub enum Operand {
    Reg(Register),
    Value(i64),
}

pub enum Instruction {
    Cpy(Operand, Register),
    Inc(Register),
    Dec(Register),
    Jnz(Register, i64),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_from_str_test_1() {
        assert_eq!("a".parse::<Register>().unwrap(), Register::A);
    }

    #[test]
    fn register_from_str_test_2() {
        assert!("p".parse::<Register>().is_err());
    }
}
