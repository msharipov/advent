use sscanf::sscanf;
use std::{collections::HashMap, ops::AddAssign, str::FromStr};

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

#[derive(Debug, PartialEq, Default)]
pub struct State {
    vars: HashMap<String, i64>,
}

impl State {
    pub fn apply(&mut self, inst: &Instruction) {
        if !self.vars.contains_key(&inst.reg) {
            self.vars.insert(inst.reg.to_owned(), 0);
        }
        let cond_reg_value = self.vars.get(&inst.cond_reg).unwrap_or(&0);
        let execute = match inst.cond {
            Comparison::Equal => cond_reg_value == &inst.cond_value,
            Comparison::NotEqual => cond_reg_value != &inst.cond_value,
            Comparison::Less => cond_reg_value < &inst.cond_value,
            Comparison::LessOrEqual => cond_reg_value <= &inst.cond_value,
            Comparison::Greater => cond_reg_value > &inst.cond_value,
            Comparison::GreaterOrEqual => cond_reg_value >= &inst.cond_value,
        };
        if execute {
            self.vars
                .get_mut(&inst.reg)
                .unwrap()
                .add_assign(&inst.change);
        }
    }

    pub fn max_register(&self) -> Option<String> {
        Some(self.vars.iter().max_by(|x, y| x.1.cmp(y.1))?.0.to_owned())
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

    #[test]
    fn state_apply_test_1() {
        let mut state = State::default();
        state.apply(&Instruction {
            reg: "abc".to_owned(),
            change: 13,
            cond_reg: "def".to_owned(),
            cond: Comparison::Greater,
            cond_value: -5,
        });
        state.apply(&Instruction {
            reg: "qwe".to_owned(),
            change: -6,
            cond_reg: "abc".to_owned(),
            cond: Comparison::Greater,
            cond_value: 3,
        });
        state.apply(&Instruction {
            reg: "abc".to_owned(),
            change: -15,
            cond_reg: "qwe".to_owned(),
            cond: Comparison::Equal,
            cond_value: -6,
        });
        state.apply(&Instruction {
            reg: "abc".to_owned(),
            change: 11,
            cond_reg: "abc".to_owned(),
            cond: Comparison::NotEqual,
            cond_value: -2,
        });
        state.apply(&Instruction {
            reg: "xyz".to_owned(),
            change: 10,
            cond_reg: "ghj".to_owned(),
            cond: Comparison::Less,
            cond_value: 1,
        });
        state.apply(&Instruction {
            reg: "qwe".to_owned(),
            change: 5,
            cond_reg: "xyz".to_owned(),
            cond: Comparison::GreaterOrEqual,
            cond_value: 10,
        });
        state.apply(&Instruction {
            reg: "tyu".to_owned(),
            change: -25,
            cond_reg: "abc".to_owned(),
            cond: Comparison::LessOrEqual,
            cond_value: -2,
        });
        let correct = State {
            vars: HashMap::from_iter([
                ("abc".to_owned(), -2),
                ("qwe".to_owned(), -1),
                ("xyz".to_owned(), 10),
                ("tyu".to_owned(), -25),
            ]),
        };
        assert_eq!(correct, state);
    }

    #[test]
    fn state_max_register_test_1() {
        let state = State {
            vars: HashMap::from_iter([
                ("abc".to_owned(), -12),
                ("def".to_owned(), 45),
                ("ghi".to_owned(), 2),
                ("jkl".to_owned(), -8),
            ]),
        };
        assert_eq!(state.max_register(), Some("def".to_owned()))
    }
}
