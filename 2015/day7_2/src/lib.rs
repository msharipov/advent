use std::collections::HashMap;

enum Wire {
    Copy(String),
    Not(String),
    And(String, String),
    Or(String, String),
    LShift(String, u16),
    RShift(String, u16),
}

#[derive(Default)]
pub struct Circuit {
    state: HashMap<String, Wire>,
}

impl Circuit {
    pub fn new(s: &[&str]) -> Result<Circuit, &'static str> {
        let mut cir = Self::default();
        for wire in s {
            cir.add_wire(wire)?;
        }
        Ok(cir)
    }

    pub fn add_wire(&mut self, s: &str) -> Result<(), &'static str> {
        use Wire::*;
        let tokens: Vec<_> = s.split(' ').collect();
        if tokens.len() == 3 {
            let name = tokens[2].to_owned();
            self.state.insert(name, Copy(tokens[0].to_owned()));
            return Ok(());
        }
        if tokens.len() == 4 && tokens[0] == "NOT" {
            let name = tokens[3].to_owned();
            self.state.insert(name, Not(tokens[1].to_owned()));
            return Ok(());
        }
        if tokens.len() == 5 {
            let name = tokens[4].to_owned();
            match tokens[1] {
                "AND" => {
                    self.state
                        .insert(name, And(tokens[0].to_owned(), tokens[2].to_owned()));
                }
                "OR" => {
                    self.state
                        .insert(name, Or(tokens[0].to_owned(), tokens[2].to_owned()));
                }
                "RSHIFT" => {
                    let shift: u16 = match tokens[2].parse() {
                        Ok(i) => Ok(i),
                        Err(_) => Err("cannot parse shift length"),
                    }?;
                    self.state.insert(name, RShift(tokens[0].to_owned(), shift));
                }
                "LSHIFT" => {
                    let shift: u16 = match tokens[2].parse() {
                        Ok(i) => Ok(i),
                        Err(_) => Err("cannot parse shift length"),
                    }?;
                    self.state.insert(name, LShift(tokens[0].to_owned(), shift));
                }
                _ => return Err("invalid operator"),
            };
            return Ok(());
        }
        Err("invalid command")
    }

    pub fn eval(&self, name: &str) -> Result<u16, String> {
        let mut table: HashMap<String, u16> = HashMap::new();
        self.eval_with_table(name, &mut table)
    }

    fn eval_with_table(&self, name: &str, table: &mut HashMap<String, u16>) -> Result<u16, String> {
        use Wire::*;
        if let Ok(c) = name.parse::<u16>() {
            return Ok(c);
        }
        if let Some(&c) = table.get(name) {
            return Ok(c);
        }
        let wire = match self.state.get(name) {
            Some(w) => Ok(w),
            None => Err(format!("wire does not exist: {name}")),
        }?;
        let result = match wire {
            Copy(w) => self.eval_with_table(w, table),
            Not(w) => match self.eval_with_table(w, table) {
                Ok(c) => Ok(!c),
                Err(e) => Err(e),
            },
            And(w1, w2) => {
                let c1 = self.eval_with_table(w1, table)?;
                let c2 = self.eval_with_table(w2, table)?;
                Ok(c1 & c2)
            }
            Or(w1, w2) => {
                let c1 = self.eval_with_table(w1, table)?;
                let c2 = self.eval_with_table(w2, table)?;
                Ok(c1 | c2)
            }
            RShift(w, sh) => {
                let c = self.eval_with_table(w, table)?;
                Ok(c >> sh)
            }
            LShift(w, sh) => {
                let c = self.eval_with_table(w, table)?;
                Ok(c << sh)
            }
        };
        if let Ok(r) = result {
            table.insert(name.to_owned(), r);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circuit_test_1() {
        let mut cir = Circuit::default();
        cir.add_wire("8 -> a").unwrap();
        cir.add_wire("128 -> b").unwrap();
        cir.add_wire("a OR b -> c").unwrap();
        assert_eq!(Ok(136), cir.eval("c"));
    }

    #[test]
    fn circuit_test_2() {
        let mut cir = Circuit::default();
        cir.add_wire("255 -> a").unwrap();
        cir.add_wire("a LSHIFT 8 -> b").unwrap();
        cir.add_wire("NOT b -> c").unwrap();
        assert_eq!(Ok(255), cir.eval("c"));
    }

    #[test]
    fn circuit_new_test_1() {
        let wires = ["255 -> a", "a LSHIFT 8 -> b", "NOT b -> c"];
        let cir = Circuit::new(&wires).unwrap();
        assert_eq!(Ok(255), cir.eval("c"));
    }
}
