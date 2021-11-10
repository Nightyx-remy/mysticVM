use crate::vm::machine::VM;
use crate::vm::instruction::Instruction;
use crate::compiler::node::{Node, ValueNode, Operator};
use crate::compiler::compiler::compile;

mod vm;
mod assembler;
mod compiler;

fn main() {
    let ast = vec![Node::VariableDefinition("a".to_string(), Box::new(Node::BinOP(
        Box::new(Node::Value(ValueNode::U8(4))),
        Operator::PLUS,
        Box::new(Node::BinOP(
            Box::new(Node::Value(ValueNode::U8(5))),
            Operator::MULTIPLY,
            Box::new(Node::Value(ValueNode::U8(2))))
            )
    ))), Node::BinOP(Box::new(
        Node::VariableCall("a".to_string())),
                     Operator::MULTIPLY,
                     Box::new(Node::Value(ValueNode::U8(3)))
    )];
    let program = compile(ast);

    // let program = assembler::assembler::assemble(std::fs::read_to_string("res\\main.mvm").expect("Failed to read file!")).expect("Failed to assembler file!");

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
