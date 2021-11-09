use crate::vm::machine::{Register, Byte};

pub enum Instruction {
    // Register Operation
    Load(Register, Byte),
    Add(Register, Register, Register),
    Sub(Register, Register, Register),
    Mul(Register, Register, Register),
    Div(Register, Register, Register),
    Cmp(Register, Register, Register),
    // Stack Operation
    SPush(Register, Register, Register),
    SCopy(Register, Register, Register),
    SPop(Register, Register, Register),
    SRep(Register, Register, Register),
    // Flow Control
    REq(Register, Register),
    Eq(Register, Byte),
    Jump8(Byte),
    Jump16(Byte, Byte),
    RJump8(Register),
    RJump16(Register, Register),
    Halt(),
}