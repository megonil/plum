use chunk::Chunk;
use opcodes::OpCode;

use crate::value::Value;

#[macro_use]
pub mod chunk;
pub mod opcodes;

macro_rules! bin_op {
    ($self:ident, $op:tt $(,)?) => {
        {
            let b = $self.stack.pop().expect("Stack underflow");
            let a = $self.stack.pop().expect("Stack underflow");
            $self.stack.push(a $op b);
        }
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

	#[inline(always)]
	fn read_u24(&mut self, wide: bool, index: &mut usize) {
		if wide {
			self.pc += 3;
			*index = from_u24!(
				self.chunk.code[self.pc - 3],
				self.chunk.code[self.pc - 2],
				self.chunk.code[self.pc - 1]
			);
		} else {
			*index = self.read_byte() as usize;
		}
	}

	pub fn execute(&mut self) {
		let mut is_wide: bool = false;
		let mut index: usize = 0;

		loop {
			if self.pc >= self.chunk.code.len() {
				break;
			}
			let byte = self.read_byte();

			if let Some(instruction) = OpCode::n(byte) {
				match instruction {
					OpCode::Constant => {
						self.read_u24(is_wide, &mut index);

						self.stack.push(self.chunk.constants[index]);
					}
					OpCode::Wide => {
						is_wide = true;
						continue;
					}

					OpCode::Add => bin_op!(self,+),
					OpCode::Sub => bin_op!(self,-),
					OpCode::Mul => bin_op!(self,*),
					OpCode::Div => bin_op!(self,/),
					OpCode::Mod => bin_op!(self,%),

					OpCode::IDiv => {
						let b = self.stack.pop();
						let a = self.stack.pop();
					}
					OpCode::Pow => {
						let b = self.stack.pop().expect("Stack underflow");
						let a = self.stack.pop().expect("Stack underflow");

						let result = a.pow(b);
						if let Some(r) = result {
							self.stack.push(r);
						} else {
							panic!();
						}
					}
					OpCode::Jmp => todo!(),
					OpCode::Jmpf => todo!(),
					OpCode::Return => todo!(),
					OpCode::Print => {
						println!("{}", self.stack.pop().expect("Stack underflow"));
					}
				}

				is_wide = false;
			}
		}
	}
}
