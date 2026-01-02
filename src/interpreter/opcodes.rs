use enumn::N;
use strum_macros::{Display, EnumIs};

#[derive(EnumIs, Display, Clone, Copy, N)]
pub enum OpCode {
	// main// push from chunk.constants[arg]
	Constant = 1,
	Wide,
	Pop,

	// operators
	// share the same logic:
	// b pop(); a = pop(); push(a op b)
	Add,  // +
	Sub,  // -
	Mul,  // *
	Div,  // /
	IDiv, // //
	Mod,  // %
	Pow,  // ^

	// control flows
	Jmp, //
	Jmpf,

	// "keywords"
	Return,
	Print, // for now this is opcode
}
