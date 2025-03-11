use sscanf::sscanf;

#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    A,
    B,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i64),
    Jie(Register, i64),
    Jio(Register, i64),
}

impl Instruction {
    fn parse(line: &str) -> Result<Self, String> {
        let hlf_parsed = sscanf!(line, "hlf {:/a|b/}", String);
        if let Ok(r) = hlf_parsed {
            if r == "a" {
                return Ok(Self::Hlf(Register::A));
            } else {
                return Ok(Self::Hlf(Register::B));
            }
        }

        let tpl_parsed = sscanf!(line, "tpl {:/a|b/}", String);
        if let Ok(r) = tpl_parsed {
            if r == "a" {
                return Ok(Self::Tpl(Register::A));
            } else {
                return Ok(Self::Tpl(Register::B));
            }
        }

        let inc_parsed = sscanf!(line, "inc {:/a|b/}", String);
        if let Ok(r) = inc_parsed {
            if r == "a" {
                return Ok(Self::Inc(Register::A));
            } else {
                return Ok(Self::Inc(Register::B));
            }
        }

        let jmp_parsed = sscanf!(line, "jmp {:/\\+|\\-/}{}", String, u64);
        if let Ok(offset) = jmp_parsed {
            if offset.0 == "+" {
                return Ok(Self::Jmp(offset.1 as i64));
            } else {
                return Ok(Self::Jmp(-(offset.1 as i64)));
            }
        }

        let jie_parsed = sscanf!(line, "jie {:/a|b/}, {:/\\+|\\-/}{}", String, String, u64);
        if let Ok(args) = jie_parsed {
            let register = if args.0 == "a" {
                Register::A
            } else {
                Register::B
            };
            let offset = if args.1 == "+" {
                args.2 as i64
            } else {
                -(args.2 as i64)
            };
            return Ok(Self::Jie(register, offset));
        }

        let jio_parsed = sscanf!(line, "jio {:/a|b/}, {:/\\+|\\-/}{}", String, String, u64);
        if let Ok(args) = jio_parsed {
            let register = if args.0 == "a" {
                Register::A
            } else {
                Register::B
            };
            let offset = if args.1 == "+" {
                args.2 as i64
            } else {
                -(args.2 as i64)
            };
            return Ok(Self::Jio(register, offset));
        }

        Err(format!("invalid instruction: \"{}\"", line.to_owned()))
    }
}

#[derive(Debug, Default, PartialEq)]
pub struct Computer {
    a: u64,
    b: u64,
    instructions: Vec<Instruction>,
    iptr: u64,
}

impl Computer {
    pub fn new(instructions: &[Instruction]) -> Self {
        Computer {
            instructions: instructions.to_vec(),
            ..Computer::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn instruction_parse_test_1() {
        use Instruction::*;
        use Register::*;
        let lines = &[
            "hlf a",
            "tpl b",
            "inc a",
            "jmp -28",
            "jie b, +4",
            "jio a, -9",
        ];
        let parsed = lines
            .iter()
            .map(|&line| Instruction::parse(line))
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        let correct = vec![Hlf(A), Tpl(B), Inc(A), Jmp(-28), Jie(B, 4), Jio(A, -9)];
        assert_eq!(correct, parsed);
    }

    #[test]
    fn instruction_parse_test_2() {
        let lines = &[
            "hlf a",
            "tpl b",
            "inc c",
            "jmp -28",
            "jie b, +4",
            "jio a, -9",
        ];
        let parsed = lines
            .iter()
            .map(|&line| Instruction::parse(line))
            .collect::<Result<Vec<_>, _>>();
        assert_eq!(parsed.unwrap_err(), "invalid instruction: \"inc c\"");
    }
}
