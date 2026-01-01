use interpreter::{VM, chunk::Chunk, opcodes::OpCode};
use value::Value;

mod interpreter;
mod lexer;
mod parser;
mod value;

fn main() {
	let mut chunk = Chunk::new();
	chunk.write_constant(Value::Int(10));
	chunk.write_constant(Value::Int(20));
	chunk.emit_byte(OpCode::Add);
	chunk.emit_byte(OpCode::Print);

	chunk.disassemble();

	let mut vm = VM::new(&chunk);
	vm.execute();
}
