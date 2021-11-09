/*
Goal: Convert an input file to a Vec of instructions.
    - Read Hexadecimal, Decimal, Binary
    - Read Register
    - Process Labels

Format:
    - 1 Instruction per line
    - If line start with # then ignore it
    - Each part of an instruction is separated by a space

Eg: LOAD r2 0x10

Syntax:
    - 0xXX -> Hexadecimal
    - 0dXX -> Decimal
    - 0bXX -> Binary
    - $X -> Label
    - rX -> Register

Keywords:
    - NEXT0 -> First Byte of next instruction
    - NEXT1 -> Second Byte of next instruction
 */

use crate::vm::instruction::Instruction;
use std::num::ParseIntError;
use std::str::Split;
use std::fmt::{Debug, Formatter};

pub enum AssemblerError {
    ParseIntError(ParseIntError),
    MissingArgument,
    WrongArgument,
    UnknownInstruction
}

impl Debug for AssemblerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssemblerError::ParseIntError(err) => write!(f, "{:?}", err)?,
            AssemblerError::MissingArgument => write!(f, "Missing Argument")?,
            AssemblerError::WrongArgument => write!(f, "Wrong Argument")?,
            AssemblerError::UnknownInstruction => write!(f, "Unknown Instruction")?,
        }
        Ok(())
    }
}

pub enum Argument {
    Byte(u8),
    Register(u8),
}

fn get_value(parts: &mut Split<&str>, instruction: usize) -> Result<Argument, AssemblerError> {
    if let Some(text) = parts.next() {
        if text == "NEXT0" {
            let address = instruction + 1;
            return Ok(Argument::Byte(((address << 8) & 0xFF) as u8));
        } else if text == "NEXT1" {
            let address = instruction + 1;
            return Ok(Argument::Byte((address & 0xFF) as u8));
        } else if text.starts_with("0x") {
            return match u8::from_str_radix(&text[2..text.len()], 16) {
                Ok(result) => Ok(Argument::Byte(result)),
                Err(err) => Err(AssemblerError::ParseIntError(err)),
            }
        } else if text.starts_with("0b") {
            return match u8::from_str_radix(&text[2..text.len()], 2) {
                Ok(result) => Ok(Argument::Byte(result)),
                Err(err) => Err(AssemblerError::ParseIntError(err)),
            }
        } else if text.starts_with("0d") {
            return match u8::from_str_radix(&text[2..text.len()], 10) {
                Ok(result) => Ok(Argument::Byte(result)),
                Err(err) => Err(AssemblerError::ParseIntError(err)),
            }
        } else if text.starts_with("r") {
            return match u8::from_str_radix(&text[1..text.len()], 16) {
                Ok(result) => Ok(Argument::Register(result)),
                Err(err) => Err(AssemblerError::ParseIntError(err)),
            }
        } else if text.starts_with("$") {
            // Label
            todo!()
        } else {
            todo!()
        }
    } else {
        return Err(AssemblerError::MissingArgument)
    }
}

pub fn assemble(source: String) -> Result<Vec<Instruction>, AssemblerError> {
    let mut program = vec![];
    let mut instruction = 0;
    for line in source.lines() {
        let mut parts = line.split(" ");
        if let Some(part1) = parts.next() {
            if part1.starts_with("$") {
                // Creation of label
                todo!()
            }

            match part1 {
                "LOAD" => {
                    if let Argument::Register(reg) = get_value(&mut parts, instruction)? {
                        if let Argument::Byte(value) = get_value(&mut parts, instruction)? {
                            program.push(Instruction::Load(reg, value));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "ADD" => {
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::Add(reg_result, reg_a, reg_b));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "SUB" => {
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::Sub(reg_result, reg_a, reg_b));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "MUL" => {
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::Mul(reg_result, reg_a, reg_b));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }}
                "DIV" => {
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::Div(reg_result, reg_a, reg_b));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "CMP" => {
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::Add(reg_result, reg_a, reg_b));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "SPUSH" => {
                    if let Argument::Register(reg_addr1) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_addr2) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_value) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::SPush(reg_addr1, reg_addr2, reg_value));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "SCOPY" => {
                    if let Argument::Register(reg_addr1) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_addr2) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_value) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::SCopy(reg_addr1, reg_addr2, reg_value));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "SPOP" => {
                    if let Argument::Register(reg_addr1) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_addr2) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_value) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::SPop(reg_addr1, reg_addr2, reg_value));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "SREP" => {
                    if let Argument::Register(reg_addr1) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_addr2) = get_value(&mut parts, instruction)? {
                            if let Argument::Register(reg_value) = get_value(&mut parts, instruction)? {
                                program.push(Instruction::SRep(reg_addr1, reg_addr2, reg_value));
                            } else {
                                return Err(AssemblerError::WrongArgument);
                            }
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "REQ" => {
                    if let Argument::Register(reg_a) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg_b) = get_value(&mut parts, instruction)? {
                            program.push(Instruction::REq(reg_a, reg_b));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "EQ" => {
                    if let Argument::Register(reg_a) = get_value(&mut parts, instruction)? {
                        if let Argument::Byte(value) = get_value(&mut parts, instruction)? {
                            program.push(Instruction::Eq(reg_a, value));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "JUMP16" => {
                    if let Argument::Byte(addr1) = get_value(&mut parts, instruction)? {
                        if let Argument::Byte(addr2) = get_value(&mut parts, instruction)? {
                            program.push(Instruction::Jump16(addr1, addr2));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "RJUMP16" => {
                    if let Argument::Register(reg1) = get_value(&mut parts, instruction)? {
                        if let Argument::Register(reg2) = get_value(&mut parts, instruction)? {
                            program.push(Instruction::RJump16(reg1, reg2));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "HALT" => program.push(Instruction::Halt()),
                &_ => return Err(AssemblerError::UnknownInstruction)
            }
        }
        instruction += 1;
    }
    return Ok(program);
}