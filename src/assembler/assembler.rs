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
    - $X0 -> Label (First Byte)
    - $X1 -> Label (Second Byte)
    - rX -> Register

Keywords:
    - NEXT0 -> First Byte of next instruction
    - NEXT1 -> Second Byte of next instruction
 */

use crate::vm::instruction::Instruction;
use std::num::ParseIntError;
use std::str::Split;
use std::fmt::{Debug, Formatter};
use std::collections::HashMap;

pub enum AssemblerError {
    ParseIntError(ParseIntError),
    MissingArgument,
    WrongArgument,
    UnknownInstruction,
    LabelNotFound,
}

impl Debug for AssemblerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            AssemblerError::ParseIntError(err) => write!(f, "{:?}", err)?,
            AssemblerError::MissingArgument => write!(f, "Missing Argument")?,
            AssemblerError::WrongArgument => write!(f, "Wrong Argument")?,
            AssemblerError::UnknownInstruction => write!(f, "Unknown Instruction")?,
            AssemblerError::LabelNotFound => write!(f, "Label Not Found")?,
        }
        Ok(())
    }
}

pub enum Argument {
    Byte(u8),
    Register(u8),
}

fn get_value(parts: &mut Split<&str>, instruction: usize, arg_number: usize, used_labels: &mut Vec<(String, usize, usize, usize)>) -> Result<Argument, AssemblerError> {
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
            if text.ends_with("0") {
                used_labels.push(((&text[1..(text.len() - 1)]).to_string(), 0, instruction, arg_number));
            } else if text.ends_with("1") {
                used_labels.push(((&text[1..(text.len() - 1)]).to_string(), 1, instruction, arg_number));
            } else {
                return Err(AssemblerError::WrongArgument);
            }
            return Ok(Argument::Byte(0));
        } else {
            return Err(AssemblerError::WrongArgument);
        }
    } else {
        return Err(AssemblerError::MissingArgument);
    }
}

pub fn assemble(source: String) -> Result<Vec<Instruction>, AssemblerError> {
    let mut program = vec![];
    let mut instruction = 0;
    let mut labels = HashMap::new();
    let mut used_labels: Vec<(String, usize, usize, usize)> = vec![];

    for line in source.lines() {
        let mut parts = line.split(" ");
        if let Some(mut part1) = parts.next() {
            if part1.starts_with("$") {
                labels.insert(&part1[1..part1.len()], instruction);
                if let Some(part2) = parts.next() {
                    part1 = part2;
                }
            }

            match part1 {
                "LOAD" => {
                    if let Argument::Register(reg) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Byte(value) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            program.push(Instruction::Load(reg, value));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "ADD" => {
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_result) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_a) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_b) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_addr1) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_addr2) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_value) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_addr1) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_addr2) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_value) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_addr1) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_addr2) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_value) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_addr1) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_addr2) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            if let Argument::Register(reg_value) = get_value(&mut parts, instruction, 2, &mut used_labels)? {
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
                    if let Argument::Register(reg_a) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg_b) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            program.push(Instruction::REq(reg_a, reg_b));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "EQ" => {
                    if let Argument::Register(reg_a) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Byte(value) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            program.push(Instruction::Eq(reg_a, value));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "JUMP16" => {
                    if let Argument::Byte(addr1) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Byte(addr2) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
                            program.push(Instruction::Jump16(addr1, addr2));
                        } else {
                            return Err(AssemblerError::WrongArgument);
                        }
                    } else {
                        return Err(AssemblerError::WrongArgument);
                    }
                }
                "RJUMP16" => {
                    if let Argument::Register(reg1) = get_value(&mut parts, instruction, 0, &mut used_labels)? {
                        if let Argument::Register(reg2) = get_value(&mut parts, instruction, 1, &mut used_labels)? {
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

    for (label, b, i, arg) in used_labels {
        if let Some(ptr) = labels.get(label.as_str()) {
            let addr = match b {
                0 => (ptr << 8) & 0xFF,
                1 => ptr & 0xFF,
                _ => panic!()
            } as u8;
            if let Some(instruction) = program.get_mut(i) {
                match instruction {
                    Instruction::Load(_, arg1) => {
                        match arg {
                            1 => *arg1 = addr,
                            _ => panic!()
                        }
                    }
                    Instruction::Add(_, _, _) => panic!(),
                    Instruction::Sub(_, _, _) => panic!(),
                    Instruction::Mul(_, _, _) => panic!(),
                    Instruction::Div(_, _, _) => panic!(),
                    Instruction::Cmp(_, _, _) => panic!(),
                    Instruction::SPush(_, _, _) => panic!(),
                    Instruction::SCopy(_, _, _) => panic!(),
                    Instruction::SPop(_, _, _) => panic!(),
                    Instruction::SRep(_, _, _) => panic!(),
                    Instruction::REq(_, arg1) => {
                        match arg {
                            1 => *arg1 = addr,
                            _ => panic!()
                        }
                    }
                    Instruction::Eq(_, _) => panic!(),
                    Instruction::Jump16(arg0, arg1) => {
                        match arg {
                            0 => *arg0 = addr,
                            1 => *arg1 = addr,
                            _ => panic!()
                        }
                    }
                    Instruction::RJump16(_, _) => panic!(),
                    Instruction::Halt() => panic!(),
                }
            }
        } else {
            return Err(AssemblerError::LabelNotFound);
        }
    }

    return Ok(program);
}