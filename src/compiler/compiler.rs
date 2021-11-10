use crate::compiler::node::{Node, ValueNode, Operator};
use crate::vm::instruction::Instruction;
use crate::vm::machine::{REGISTERS, STACK_SIZE, IGNORE};
use std::collections::HashMap;

fn compile_current(program: &mut Vec<Instruction>, registers: &mut [bool; REGISTERS], node: &Node, memory_map: &mut Vec<(usize, usize)>, variable_dictionary: &mut HashMap<String, (u8, u8)>) -> Vec<u8> {
    match node {
        Node::Value(value_node) => {
            match value_node {
                ValueNode::U8(value) => {
                    for i in 0..REGISTERS {
                        if registers[i] {
                            registers[i] = false;
                            program.push(Instruction::Load(i as u8, value.clone()));
                            return vec![i as u8];
                        }
                    }
                    panic!()
                }
            }
        }
        Node::BinOP(left, op, right) => {
            let (used_register1, used_register2) = if left.get_weight() >= right.get_weight() {
                let used_register1 = compile_current(program, registers, left, memory_map, variable_dictionary);
                let used_register2 = compile_current(program, registers, right, memory_map, variable_dictionary);
                (used_register1, used_register2)
            } else {
                let used_register2 = compile_current(program, registers, right, memory_map, variable_dictionary);
                let used_register1 = compile_current(program, registers, left, memory_map, variable_dictionary);
                (used_register1, used_register2)
            };
            for i in 0..REGISTERS {
                if registers[i] {
                    registers[i] = false;
                    match op {
                        Operator::PLUS => program.push(Instruction::Add(i as u8, used_register1[0], used_register2[0])),
                        Operator::MINUS => program.push(Instruction::Sub(i as u8, used_register1[0], used_register2[0])),
                        Operator::MULTIPLY => program.push(Instruction::Mul(i as u8, used_register1[0], used_register2[0])),
                        Operator::DIVIDE => program.push(Instruction::Div(i as u8, used_register1[0], used_register2[0])),
                    }
                    registers[used_register1[0] as usize] = true;
                    registers[used_register2[0] as usize] = true;
                    return vec![i as u8];
                }
            }
            panic!();
        }
        Node::VariableDefinition(name, value) => {
            let value = compile_current(program, registers, value, memory_map, variable_dictionary);
            if variable_dictionary.get(name).is_none() {
                let map = memory_map.get_mut(0).unwrap();
                let addr1 = ((map.0 >> 8) & 0xFF) as u8;
                let addr2 = (map.0 & 0xFF) as u8;
                if map.1 > 1 {
                    map.1 -= 1;
                    map.0 += 1;
                } else {
                    memory_map.remove(0);
                }

                variable_dictionary.insert(name.clone(), (addr1, addr2));
                program.push(Instruction::SPush(IGNORE, IGNORE, value[0]));
                return vec![];
            } else {
                panic!()
            }
        }
        Node::VariableCall(name) => {
            if let Some(var) = variable_dictionary.get(name) {
                let mut reg1: Option<u8> = None;
                let mut reg2: Option<u8> = None;
                let mut reg3: Option<u8> = None;
                for i in 0..REGISTERS {
                    if registers[i] {
                        registers[i] = false;
                        if reg1.is_none() {
                            reg1 = Some(i as u8);
                            program.push(Instruction::Load(i as u8, var.0));
                        } else if reg2.is_none() {
                            reg2 = Some(i as u8);
                            program.push(Instruction::Load(i as u8, var.1));
                        } else {
                            reg3 = Some(i as u8);
                            program.push(Instruction::SCopy(reg1.unwrap(), reg2.unwrap(), reg3.unwrap()));
                            return vec![reg3.unwrap(), reg1.unwrap(), reg2.unwrap()];
                        }
                    }
                }
                panic!()
            } else {
                panic!()
            }
        }
    }
}

pub fn compile(ast: Vec<Node>) -> Vec<Instruction> {
    let mut program = vec![];
    let mut registers = [true; REGISTERS];
    let mut memory_map: Vec<(usize, usize)> = vec![(0, STACK_SIZE)];
    let mut variable_dictionary: HashMap<String, (u8, u8)> = HashMap::new();

    for node in ast.iter() {
        compile_current(&mut program, &mut registers, node, &mut memory_map, &mut variable_dictionary);
        registers.fill(true); // Free All registers
    }

    return program;
}