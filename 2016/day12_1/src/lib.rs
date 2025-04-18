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

impl FromStr for Instruction {
    type Err = sscanf::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((x, y)) = sscanf!(s, "cpy {:/a|b|c|d/} {:/a|b|c|d/}", &str, &str) {
            let x = x.parse::<Register>()?;
            let y = y.parse::<Register>()?;
            return Ok(Instruction::Cpy(Operand::Reg(x), y));
        }
        if let Ok((x, y)) = sscanf!(s, "cpy {i64} {:/a|b|c|d/}", &str) {
            let y = y.parse::<Register>()?;
            return Ok(Instruction::Cpy(Operand::Value(x), y));
        }
        if let Ok(reg) = sscanf!(s, "inc {:/a|b|c|d/}", &str) {
            let reg = reg.parse::<Register>()?;
            return Ok(Instruction::Inc(reg));
        }
        if let Ok(reg) = sscanf!(s, "dec {:/a|b|c|d/}", &str) {
            let reg = reg.parse::<Register>()?;
            return Ok(Instruction::Dec(reg));
        }
        if let Ok((reg, jump_len)) = sscanf!(s, "jnz {:/a|b|c|d/} {i64}", &str) {
            let reg = reg.parse::<Register>()?;
            return Ok(Instruction::Jnz(reg, jump_len));
        }
        Err(Self::Err::MatchFailed)
    }
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
