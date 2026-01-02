use chunk::Chunk;
use opcodes::OpCode;

use crate::{
	error::{RuntimeError::*, RuntimeResult},
	value::{PlumInt, Value},
};

#[macro_use]
pub mod chunk;
pub mod opcodes;

macro_rules! bin_op {
	($self:ident, $op:tt $(,)?) => {{
		let b = safepop!($self);
		let a = safepop!($self);

		$self.stack.push(a $op b);
	}};
}

macro_rules! safepop {
	($self:ident) => {
		$self
			.stack
			.pop()
			.ok_or(StackUnderflow($self.current_line()))?
	};
}

pub struct VM<'a> {
	chunk: &'a Chunk,
	pc: usize,
	stack: Vec<Value>,
}
impl<'a> VM<'a> {
	pub fn new(chunk: &'a Chunk) -> Self {
		Self {
			chunk,
			pc: 0,
			stack: Vec::new(),
		}
	}

	#[inline(always)]
	fn read_byte(&mut self) -> u8 {
		self.pc += 1;
		self.chunk.code[self.pc - 1]
	}

	fn read_u24(&mut self, wide: bool) -> u32 {
		if wide {
			self.pc += 3;

			from_u24!(
				self.chunk.code[self.pc - 3],
				self.chunk.code[self.pc - 2],
				self.chunk.code[self.pc - 1]
			) as u32
		} else {
			self.read_byte() as u32
		}
	}

	#[inline(always)]
	fn read_i24(&mut self, wide: bool) -> i32 {
		let u = self.read_u24(wide);

		((u << 8) as i32) >> 8
	}

	fn read_u16(&mut self) -> u16 {
		self.pc += 2;
		let major = self.chunk.code[self.pc - 2] as u16;
		let minor = self.chunk.code[self.pc - 1] as u16;

		major | (minor << 8)
	}

	#[inline(always)]
	fn read_i16(&mut self) -> i16 {
		self.read_u16() as i16
	}

	#[inline(always)]
	fn current_line(&self) -> usize {
		// placeholder
		0
	}

	pub fn execute(&mut self) -> RuntimeResult<()> {
		let mut is_wide: bool = false;
		let mut index: u32 = 0; // we can't got more than u32 

		loop {
			if self.pc >= self.chunk.code.len() {
				if self.stack.len() != 0 {
					eprintln!("warning: stack isn't empty at the end of the execution");
				}
				return Ok(());
			}
			let byte = self.read_byte();

			if let Some(instruction) = OpCode::n(byte) {
				match instruction {
					OpCode::Constant => {
						index = self.read_u24(is_wide);

						self.stack.push(self.chunk.constants[index as usize]);
					}

					OpCode::Wide => {
						is_wide = true;
						continue;
					}

					OpCode::Add => bin_op!(self,+),
					OpCode::Sub => bin_op!(self,-),
					OpCode::Mul => bin_op!(self,*),
					OpCode::Mod => bin_op!(self,%),

					OpCode::Div => {
						let b = safepop!(self);
						let a = safepop!(self);

						let result = a / b;
						self.stack.push(result?);
					}

					OpCode::IDiv => {
						let b = safepop!(self);
						let a = safepop!(self);

						let result = (a / b)?;

						let int_val: PlumInt = match result {
							Value::Num(x) => x as PlumInt,
							Value::Int(a) => a,
						};

						self.stack.push(Value::Int(int_val));
					}

					OpCode::Pow => {
						let b = safepop!(self);
						let a = safepop!(self);

						let result = a.pow(b)?;
						self.stack.push(result);
					}

					OpCode::Jmp => {
						let pos = self.read_i16() as isize;

						self.pc = ((self.pc as isize) + pos) as usize;
					}

					OpCode::Jmpf => {
						let pos = self.read_i16() as isize;

						if safepop!(self).into() {
							self.pc = ((self.pc as isize) + pos) as usize;
						}
					}

					OpCode::Print => {
						println!("{}", safepop!(self));
					}
					OpCode::Pop => {
						// discard the pop value
						safepop!(self);
					}

					OpCode::Return => todo!(),
				}

				is_wide = false;
			}
		}
	}
}
