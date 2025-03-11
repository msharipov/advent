#[derive(Debug, PartialEq)]
pub enum Register {
    A,
    B,
}

#[derive(Debug, PartialEq)]
pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(i64),
    Jie(Register, i64),
    Jio(Register, i64),
}

#[derive(Debug, Default, PartialEq)]
pub struct Computer {
    a: u64,
    b: u64,
    instructions: Vec<Instruction>,
    iptr: u64,
}
