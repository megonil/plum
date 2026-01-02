use error::InterpreterResult;
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

	let some_jmp = chunk.start_jump(OpCode::Jmp);

	chunk.write_constant(Value::Int(25));
	chunk.write_constant(Value::Num(0.5));

	chunk.emit_byte(OpCode::Pow);
	chunk.emit_byte(OpCode::Print);

	chunk.end_jump(some_jmp)?;

	chunk.write_constant(Value::Int(1));
	let another_jump = chunk.start_jump(OpCode::Jmpf);

	chunk.write_constant(Value::Int(1));
	chunk.emit_byte(OpCode::Print);

	chunk.end_jump(another_jump)?;

	chunk.write_constant(Value::Int(2));
	chunk.emit_byte(OpCode::Print);

	chunk.disassemble()?;

	let mut vm = VM::new(&chunk);
	vm.execute()?;

	Ok(())
}
