pub enum Register {
    A,
    B,
}

pub enum Instruction {
    Hlf(Register),
    Tpl(Register),
    Inc(Register),
    Jmp(u64),
    Jie(Register),
    Jio(Register),
}
