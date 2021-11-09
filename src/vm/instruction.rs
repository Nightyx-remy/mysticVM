use crate::vm::machine::{Register, Byte};

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
    Jump8(Byte),                            // Jump to the 8 bits address [arg0]
    Jump16(Byte, Byte),                     // Jump to the 16 bits address [arg0][arg1]
    RJump8(Register),                       // Jump to the 8 bits address stored in register [arg0]
    RJump16(Register, Register),            // Jump to the 16 bits address stored in registers [arg0][arg1]
    Halt(),                                 // Pause the program (Usually End of Program)
}