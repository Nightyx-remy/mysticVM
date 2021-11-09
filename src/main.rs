use crate::vm::machine::VM;
use crate::vm::instruction::Instruction;

mod vm;

fn main() {
    let mut vm = VM::new(vec![
        Instruction::Load(0x2, 0x10),
        Instruction::SPush(0x0, 0x1, 0x2),
        Instruction::Load(0x2, 0x04),
        Instruction::SCopy(0x0, 0x1, 0x3),
        Instruction::Add(0x2, 0x2, 0x3),
        Instruction::SRep(0x0, 0x1, 0x2)
    ]);
    vm.run();

    println!("--- Registers ---");
    vm.print_registers();

    println!("\n--- Stack ---");
    vm.print_memory(4);
}
