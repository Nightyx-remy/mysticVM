use crate::vm::machine::{Register, Byte, REGISTERS, IGNORE};
use std::fmt::{Debug, Formatter};

pub enum Instruction {
    // Register Operation
    Load(Register, Byte),                   // Load the value [arg1] to the register [arg0]
    Add(Register, Register, Register),      // Add the registers [arg1] and [arg2] and put the result in register [arg0]
    Sub(Register, Register, Register),      // Subtract the registers [arg1] and [arg2] and put the result in register [arg0]
    Mul(Register, Register, Register),      // Multiply the registers [arg1] and [arg2] and put the result in register [arg0]
    Div(Register, Register, Register),      // Divide the registers [arg1] and [arg2] and put the result in register [arg0]
    Cmp(Register, Register, Register),      // Compare the registers [arg1] and [arg2] and put the result in register [arg0] (0 -> [arg1] < [arg2], 1 -> [arg1] == [arg2], 2 -> [arg1] > [arg2])
    // Stack Operation
    SPush(Register, Register, Register),    // Push the register [arg2] to the stack and put the address in [arg0][arg1]
    SCopy(Register, Register, Register),    // Copy the value at address [arg0][arg1] and put it in the register [arg2]
    SPop(Register, Register, Register),     // Pop the value at address [arg0][arg1] and put it in the register [arg2]
    SRep(Register, Register, Register),     // Replace the value at address [arg0][arg1] byt the register [arg2]
    // Flow Control
    REq(Register, Register),                // Skip the next instruction if the register [arg0] != to the register [arg1]
    Eq(Register, Byte),                     // Skip the next instruction if the register [arg0] != to the value [arg1]
    Jump16(Byte, Byte),                     // Jump to the 16 bits address [arg0][arg1]
    RJump16(Register, Register),            // Jump to the 16 bits address stored in registers [arg0][arg1]
    Halt(),                                 // Pause the program (Usually End of Program)
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Load(a, b) => write!(f, "LOAD r{:X} 0x{:02X}", a, b)?,
            Instruction::Add(a, b, c) => write!(f, "ADD r{:X} r{:X} r{:X}", a, b, c)?,
            Instruction::Sub(a, b, c) => write!(f, "SUB r{:X} r{:X} r{:X}", a, b, c)?,
            Instruction::Mul(a, b, c) => write!(f, "MUL r{:X} r{:X} r{:X}", a, b, c)?,
            Instruction::Div(a, b, c) => write!(f, "DIV r{:X} r{:X} r{:X}", a, b, c)?,
            Instruction::Cmp(a, b, c) => write!(f, "CMP r{:X} r{:X} r{:X}", a, b, c)?,
            Instruction::SPush(a, b, c) => {
                write!(f, "SPUSH ")?;
                if *a >= IGNORE {
                    write!(f, "_ ")?;
                } else {
                    write!(f, "r{:X}", a)?;
                }
                if *b >= IGNORE {
                    write!(f, "_ ")?;
                } else {
                    write!(f, "r{:X}", b)?;
                }
                write!(f, "r{:X}", c)?;
            },
            Instruction::SCopy(a, b, c) => write!(f, "SCOPY r{:X} r{:X} r{:X}", a, b, c)?,
            Instruction::SPop(a, b, c) => write!(f, "SPOP r{:X} r{:X} r{:X}", a, b, c)?,
            Instruction::SRep(a, b, c) => write!(f, "SREP r{:X} r{:X} r{:X}", a, b, c)?,
            Instruction::REq(a, b) => write!(f, "REQ r{:X} r{:X}", a, b)?,
            Instruction::Eq(a, b) => write!(f, "REQ r{:X} 0x{:02X}", a, b)?,
            Instruction::Jump16(a, b) => write!(f, "JUMP16 0x{:02X} 0x{:02X}", a, b)?,
            Instruction::RJump16(a, b) => write!(f, "RJUMP16 r{:X} r{:X}", a, b)?,
            Instruction::Halt() => write!(f, "HALT")?,
        }
        Ok(())
    }
}