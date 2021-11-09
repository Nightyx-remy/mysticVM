use crate::vm::machine::VM;
use crate::vm::instruction::Instruction;

mod vm;
mod assembler;

fn main() {
    let program = assembler::assembler::assemble(std::fs::read_to_string("res\\main.mvm").expect("Failed to read file!")).expect("Failed to assembler file!");

    println!("--- Program ---");
    for instruction in program.iter() {
        println!("{:?}", instruction);
    }

    let mut vm = VM::new(program);
    vm.run();

    println!("\n--- Registers ---");
    vm.print_registers();

    println!("\n--- Stack ---");
    vm.print_memory(4);
}
