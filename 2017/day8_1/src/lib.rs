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
}
