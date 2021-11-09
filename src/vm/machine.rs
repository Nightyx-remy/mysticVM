use crate::vm::instruction::Instruction;
use std::cmp::min;

pub type Register = u8;
pub type Byte = u8;

const STACK_SIZE: usize = 2_usize.pow(16);
const REGISTERS: usize = 16;

pub struct VM {
    stack: [u8; STACK_SIZE],
    // (ptr, size)
    stack_memory_map: Vec<(usize, usize)>,
    registers: [u8; REGISTERS],
    program: Vec<Instruction>,
    program_counter: usize,
}

impl VM {

    pub fn new(program: Vec<Instruction>) -> VM {
        return VM {
            stack: [0; STACK_SIZE],
            stack_memory_map: vec![(0, STACK_SIZE)],
            registers: [0; REGISTERS],
            program,
            program_counter: 0
        }
    }

    pub fn run_once(&mut self) -> bool {
        if self.program_counter >= self.program.len() {
            return false;
        }
        match self.program[self.program_counter] {
            Instruction::Load(reg, value) => {
                self.registers[reg as usize] = value;
            }
            Instruction::Add(reg_result, reg_a, reg_b) => {
                self.registers[reg_result as usize] = self.registers[reg_a as usize] + self.registers[reg_b as usize];
            }
            Instruction::Sub(reg_result, reg_a, reg_b) => {
                self.registers[reg_result as usize] = self.registers[reg_a as usize] - self.registers[reg_b as usize];
            }
            Instruction::Mul(reg_result, reg_a, reg_b) => {
                self.registers[reg_result as usize] = self.registers[reg_a as usize] * self.registers[reg_b as usize];
            }
            Instruction::Div(reg_result, reg_a, reg_b) => {
                self.registers[reg_result as usize] = self.registers[reg_a as usize] / self.registers[reg_b as usize];
            }
            Instruction::Cmp(reg_result, reg_a, reg_b) => {
                let v_a = self.registers[reg_a as usize];
                let v_b = self.registers[reg_b as usize];
                if v_a < v_b {
                    self.registers[reg_result as usize] = 0;
                } else if v_a == v_b {
                    self.registers[reg_result as usize] = 1;
                } else {
                    self.registers[reg_result as usize] = 2;
                }
            }
            Instruction::SPush(reg_addr1, reg_addr2, reg_value) => {
                let map = self.stack_memory_map.get_mut(0).unwrap();
                self.stack[map.0] = self.registers[reg_value as usize];
                self.registers[reg_addr1 as usize] = ((map.0 >> 8) & 0xFF) as u8;
                self.registers[reg_addr2 as usize] = (map.0 & 0xFF) as u8;
                if map.1 > 1 {
                    map.1 -= 1;
                    map.0 += 1;
                } else {
                    self.stack_memory_map.remove(0);
                }
            }
            Instruction::SPop(reg_addr1, reg_addr2, reg_value) => {
                let address = (self.registers[reg_addr1 as usize] as usize) << 8 + self.registers[reg_addr2 as usize] as usize;
                let value = self.stack[address];
                self.stack_memory_map.push((address, 1));
                self.registers[reg_value as usize] = value;
            }
            Instruction::SCopy(reg_addr1, reg_addr2, reg_value) => {
                let address = (self.registers[reg_addr1 as usize] as usize) << 8 + self.registers[reg_addr2 as usize] as usize;
                let value = self.stack[address];
                self.registers[reg_value as usize] = value;
            }
            Instruction::SRep(reg_addr1, reg_addr2, reg_value) => {
                let address = (self.registers[reg_addr1 as usize] as usize) << 8 + self.registers[reg_addr2 as usize] as usize;
                self.stack[address] = self.registers[reg_value as usize];
                // TODO: Check if not used
            }
            Instruction::REq(reg1, reg2) => {
                if self.registers[reg1 as usize] != self.registers[reg2 as usize] {
                    self.program_counter += 1;
                }
            }
            Instruction::Eq(reg, value) => {
                if self.registers[reg as usize] != value {
                    self.program_counter += 1;
                }
            }
            Instruction::Jump8(value) => {
                self.program_counter = value as usize;
            }
            Instruction::Jump16(byte1, byte2) => {
                self.program_counter = (byte1 << 8 + byte2) as usize;
            }
            Instruction::RJump8(reg) => {
                self.program_counter = self.registers[reg as usize] as usize;
            }
            Instruction::RJump16(reg1, reg2) => {
                self.program_counter = (self.registers[reg1 as usize] << 8 + self.registers[reg2 as usize]) as usize;
            }
            Instruction::Halt() => return false,
        }
        self.program_counter += 1;
        return true;
    }

    pub fn run(&mut self) {
        while self.run_once() {

        }
    }

    pub fn print_registers(&mut self) {
        for i in 0..REGISTERS {
            println!("[{:X}]: 0x{:02X}", i, self.registers[i]);
        }
    }

    pub fn print_memory(&mut self, rows: usize) {
        for i in 0..min(STACK_SIZE / 16, rows) {
            print!("[{:03X}]:", i);
            for j in 0..16 {
                print!(" {:02X}", self.stack[i * 16 + j]);
            }
            println!();
        }
    }

}