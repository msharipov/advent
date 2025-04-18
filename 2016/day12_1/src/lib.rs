use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Reg(Register),
    Value(i64),
}

#[derive(Debug, PartialEq, Clone)]
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

pub fn parse_instructions(lines: &[&str]) -> Result<Vec<Instruction>, sscanf::Error> {
    lines
        .iter()
        .map(|&line| line.parse::<Instruction>())
        .collect()
}

pub struct Computer {
    iar: usize,
    instructions: Vec<Instruction>,
    ra: i64,
    rb: i64,
    rc: i64,
    rd: i64,
}

impl Computer {
    pub fn new(instructions: &[Instruction]) -> Self {
        Computer {
            iar: 0,
            ra: 0,
            rb: 0,
            rc: 0,
            rd: 0,
            instructions: instructions.iter().cloned().collect(),
        }
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

    #[test]
    fn parse_instructions_test_1() {
        use Instruction::*;
        let lines = ["cpy b a", "cpy 12 c", "dec d", "inc c", "jnz a -19"];
        let correct = vec![
            Cpy(Operand::Reg(Register::B), Register::A),
            Cpy(Operand::Value(12), Register::C),
            Dec(Register::D),
            Inc(Register::C),
            Jnz(Register::A, -19),
        ];
        assert_eq!(parse_instructions(&lines).unwrap(), correct);
    }

    #[test]
    fn parse_instructions_test_2() {
        let lines = ["cpy a 13", "cpy 12 c", "dec d", "inc c", "jnz a -19"];
        assert!(parse_instructions(&lines).is_err());
    }
}
