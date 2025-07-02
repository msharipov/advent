use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Comparison {
    Equal,
    NotEqual,
    Greater,
    GreaterOrEqual,
    Less,
    LessOrEqual,
}

impl FromStr for Comparison {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "==" => Ok(Comparison::Equal),
            "!=" => Ok(Comparison::NotEqual),
            ">" => Ok(Comparison::Greater),
            ">=" => Ok(Comparison::GreaterOrEqual),
            "<" => Ok(Comparison::Less),
            "<=" => Ok(Comparison::LessOrEqual),
            _ => Err(sscanf::Error::MatchFailed),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    reg: String,
    change: i64,
    cond_reg: String,
    cond: Comparison,
    cond_value: i64,
}

impl FromStr for Instruction {
    type Err = sscanf::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed = sscanf!(
            s,
            "{String} {&str:/inc|dec/} {i64} if {String} {&str} {i64}"
        )?;
        let (reg, op, change, cond_reg, cond, cond_value) = parsed;
        let cond = cond.parse::<Comparison>()?;
        let change = if op == "inc" { change } else { -change };
        Ok(Instruction {
            reg,
            change,
            cond_reg,
            cond,
            cond_value,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_from_str_test_1() {
        assert_eq!(
            "abc inc -17 if def == 130".parse::<Instruction>().unwrap(),
            Instruction {
                reg: "abc".to_owned(),
                change: -17,
                cond_reg: "def".to_owned(),
                cond: Comparison::Equal,
                cond_value: 130,
            }
        )
    }

    #[test]
    fn instruction_from_str_test_2() {
        assert_eq!(
            "abc dec -50 if def != 22".parse::<Instruction>().unwrap(),
            Instruction {
                reg: "abc".to_owned(),
                change: 50,
                cond_reg: "def".to_owned(),
                cond: Comparison::NotEqual,
                cond_value: 22,
            }
        )
    }

    #[test]
    fn instruction_from_str_test_3() {
        assert_eq!(
            "abc inc 55 if def > 0".parse::<Instruction>().unwrap(),
            Instruction {
                reg: "abc".to_owned(),
                change: 55,
                cond_reg: "def".to_owned(),
                cond: Comparison::Greater,
                cond_value: 0,
            }
        )
    }

    #[test]
    fn instruction_from_str_test_4() {
        assert_eq!(
            "abc dec 29 if def >= -85".parse::<Instruction>().unwrap(),
            Instruction {
                reg: "abc".to_owned(),
                change: -29,
                cond_reg: "def".to_owned(),
                cond: Comparison::GreaterOrEqual,
                cond_value: -85,
            }
        )
    }

    #[test]
    fn instruction_from_str_test_5() {
        assert_eq!(
            "abc inc 31 if def < 130".parse::<Instruction>().unwrap(),
            Instruction {
                reg: "abc".to_owned(),
                change: 31,
                cond_reg: "def".to_owned(),
                cond: Comparison::Less,
                cond_value: 130,
            }
        )
    }

    #[test]
    fn instruction_from_str_test_6() {
        assert_eq!(
            "abc inc 31 if def <= 77".parse::<Instruction>().unwrap(),
            Instruction {
                reg: "abc".to_owned(),
                change: 31,
                cond_reg: "def".to_owned(),
                cond: Comparison::LessOrEqual,
                cond_value: 77,
            }
        )
    }
}
