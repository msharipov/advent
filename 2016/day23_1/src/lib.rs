use sscanf::sscanf;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone, Copy)]
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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Operand {
    Reg(Register),
    Value(i64),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Instruction {
    Cpy(Operand, Operand),
    Inc(Register),
    Dec(Register),
    Jnz(Operand, Operand),
}

impl FromStr for Instruction {
    type Err = sscanf::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok((x, y)) = sscanf!(s, "cpy {:/a|b|c|d/} {:/a|b|c|d/}", &str, &str) {
            let x = x.parse::<Register>()?;
            let y = y.parse::<Register>()?;
            return Ok(Instruction::Cpy(Operand::Reg(x), Operand::Reg(y)));
        }
        if let Ok((x, y)) = sscanf!(s, "cpy {i64} {:/a|b|c|d/}", &str) {
            let y = y.parse::<Register>()?;
            return Ok(Instruction::Cpy(Operand::Value(x), Operand::Reg(y)));
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
            return Ok(Instruction::Jnz(
                Operand::Reg(reg),
                Operand::Value(jump_len),
            ));
        }
        if let Ok((val, jump_len)) = sscanf!(s, "jnz {i64} {i64}") {
            return Ok(Instruction::Jnz(
                Operand::Value(val),
                Operand::Value(jump_len),
            ));
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

pub struct Halt;

#[derive(Debug, PartialEq)]
pub struct Computer {
    iar: i64,
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
            instructions: instructions.to_vec(),
        }
    }

    pub fn read_reg(&self, reg: Register) -> i64 {
        match reg {
            Register::A => self.ra,
            Register::B => self.rb,
            Register::C => self.rc,
            Register::D => self.rd,
        }
    }

    fn set_reg(&mut self, reg: Register, val: i64) {
        let reg = match reg {
            Register::A => &mut self.ra,
            Register::B => &mut self.rb,
            Register::C => &mut self.rc,
            Register::D => &mut self.rd,
        };
        *reg = val;
    }

    fn cpy(&mut self, op_from: Operand, op_to: Operand) {
        if let Operand::Reg(op_to) = op_to {
            match op_from {
                Operand::Reg(reg) => {
                    self.set_reg(op_to, self.read_reg(reg));
                }
                Operand::Value(val) => {
                    self.set_reg(op_to, val);
                }
            }
        }
        self.iar += 1;
    }

    fn inc(&mut self, reg: Register) {
        self.set_reg(reg, self.read_reg(reg) + 1);
        self.iar += 1;
    }

    fn dec(&mut self, reg: Register) {
        self.set_reg(reg, self.read_reg(reg) - 1);
        self.iar += 1;
    }

    fn jnz(&mut self, cond: Operand, jump_len: Operand) {
        let val = match cond {
            Operand::Value(val) => val,
            Operand::Reg(reg) => self.read_reg(reg),
        };
        if val != 0 {
            self.iar += match jump_len {
                Operand::Value(val) => val,
                Operand::Reg(r) => self.read_reg(r),
            };
        } else {
            self.iar += 1;
        }
    }

    pub fn next_step(&mut self) -> Result<(), Halt> {
        use Instruction::*;
        if self.iar < 0 {
            return Err(Halt);
        }
        match self.instructions.get(self.iar as usize) {
            None => Err(Halt),
            Some(inst) => {
                match inst {
                    Cpy(op_from, op_to) => {
                        self.cpy(*op_from, *op_to);
                    }
                    Inc(reg) => {
                        self.inc(*reg);
                    }
                    Dec(reg) => {
                        self.dec(*reg);
                    }
                    Jnz(cond, jump_len) => {
                        self.jnz(*cond, *jump_len);
                    }
                }
                Ok(())
            }
        }
    }

    pub fn run(&mut self) {
        while self.next_step().is_ok() {}
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
        let lines = [
            "cpy b a",
            "cpy 12 c",
            "dec d",
            "inc c",
            "jnz a -19",
            "jnz 5 12",
        ];
        let correct = vec![
            Cpy(Operand::Reg(Register::B), Operand::Reg(Register::A)),
            Cpy(Operand::Value(12), Operand::Reg(Register::C)),
            Dec(Register::D),
            Inc(Register::C),
            Jnz(Operand::Reg(Register::A), Operand::Value(-19)),
            Jnz(Operand::Value(5), Operand::Value(12)),
        ];
        assert_eq!(parse_instructions(&lines).unwrap(), correct);
    }

    #[test]
    fn parse_instructions_test_2() {
        let lines = [
            "cpy a 13",
            "cpy 12 c",
            "dec d",
            "inc c",
            "jnz a -19",
            "jnz 5 12",
        ];
        assert!(parse_instructions(&lines).is_err());
    }

    #[test]
    fn cpy_test_1() {
        let mut comp = Computer::new(&[]);
        comp.set_reg(Register::A, 18);
        comp.set_reg(Register::B, -12);
        assert_eq!(comp.read_reg(Register::A), 18);
        comp.cpy(Operand::Reg(Register::B), Operand::Reg(Register::A));
        assert_eq!(comp.read_reg(Register::A), -12);
    }

    #[test]
    fn cpy_test_2() {
        let mut comp = Computer::new(&[]);
        comp.set_reg(Register::A, 18);
        assert_eq!(comp.read_reg(Register::A), 18);
        comp.cpy(Operand::Value(45), Operand::Reg(Register::A));
        assert_eq!(comp.read_reg(Register::A), 45);
    }

    #[test]
    fn inc_test_1() {
        let mut comp = Computer::new(&[]);
        comp.set_reg(Register::B, 15);
        comp.inc(Register::B);
        comp.inc(Register::B);
        comp.inc(Register::B);
        comp.inc(Register::B);
        assert_eq!(comp.read_reg(Register::B), 19);
    }

    #[test]
    fn dec_test_1() {
        let mut comp = Computer::new(&[]);
        comp.set_reg(Register::B, 15);
        comp.dec(Register::B);
        comp.dec(Register::B);
        comp.dec(Register::B);
        comp.dec(Register::B);
        assert_eq!(comp.read_reg(Register::B), 11);
    }

    #[test]
    fn jnz_test_1() {
        let mut comp = Computer::new(&[]);
        assert_eq!(comp.iar, 0);
        comp.set_reg(Register::B, 5);
        comp.jnz(Operand::Reg(Register::B), Operand::Value(14));
        assert_eq!(comp.iar, 14);
    }

    #[test]
    fn jnz_test_2() {
        let mut comp = Computer::new(&[]);
        assert_eq!(comp.iar, 0);
        comp.jnz(Operand::Reg(Register::B), Operand::Value(14));
        assert_eq!(comp.iar, 1);
    }

    #[test]
    fn jnz_test_3() {
        let mut comp = Computer::new(&[]);
        assert_eq!(comp.iar, 0);
        comp.jnz(Operand::Value(11), Operand::Value(14));
        assert_eq!(comp.iar, 14);
    }

    #[test]
    fn run_test_1() {
        let instructions = [
            "cpy 10 a", "inc b", "inc c", "inc c", "inc d", "inc d", "inc d", "dec a", "jnz a -7",
        ];
        let instructions = parse_instructions(&instructions).unwrap();
        let mut comp = Computer::new(&instructions);
        comp.run();
        assert_eq!(comp.read_reg(Register::A), 0);
        assert_eq!(comp.read_reg(Register::B), 10);
        assert_eq!(comp.read_reg(Register::C), 20);
        assert_eq!(comp.read_reg(Register::D), 30);
    }
}
