use thiserror::Error;

#[derive(Error, Debug)]
pub enum InterpreterError {
	// syntax error...
	// lexical error...
	// maybe semantic error...
	#[error("Runtime error: {0}")]
	RuntimeError(#[from] RuntimeError),

	#[error("Disassembler error: {0}")]
	DisasmError(#[from] DisasmError),
}

#[derive(Error, Debug)]
pub enum RuntimeError {
	#[error("{0}")]
	ValueError(#[from] ValueError),

	#[error("StackUnderflow at line {0}")]
	StackUnderflow(usize),

	#[error("StackOverflow at line {0}")]
	StackOverflow(usize),
}

#[derive(Error, Debug)]
pub enum DisasmError {
	#[error("UnkownConstant at `{0}` index")]
	UnkownConstant(usize),

	#[error("UnknownOpCode: ")]
	UnknownOpCode(usize),
}

#[derive(Error, Debug)]
pub enum ValueError {
	#[error("FloatPointException: division by 0")]
	FloatPointException(),
}

pub type InterpreterResult<T> = Result<T, InterpreterError>;
pub type DisasmResult<T> = Result<T, DisasmError>;
pub type RuntimeResult<T> = Result<T, RuntimeError>;
pub type ValueResult<T> = Result<T, ValueError>;
