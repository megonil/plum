use error::{InterpreterError, InterpreterResult};
use interpreter::{VM, chunk::Chunk, opcodes::OpCode};
use value::Value;

pub mod error;
mod interpreter;
mod lexer;
mod parser;
mod value;

fn main() {
	if let Err(e) = run() {
		eprintln!("{}", e);
		std::process::exit(1);
	}
}

fn run() -> InterpreterResult<()> {
	let mut chunk = Chunk::new();

	chunk.write_constant(Value::Int(10));
	chunk.write_constant(Value::Int(0));
	chunk.emit_byte(OpCode::Div);
	chunk.emit_byte(OpCode::Print);

	chunk.disassemble()?;

	let mut vm = VM::new(&chunk);
	vm.execute()?;

	Ok(())
}
