use crate::value::Value;

use super::opcodes::OpCode;

#[derive(Clone)]
pub struct Chunk {
	pub code: Vec<u8>,
	pub constants: Vec<Value>,
}

macro_rules! from_u24 {
	($major:expr, $mid:expr, $minor:expr) => {
		($major as u32 | ($mid as u32 >> 8) | ($minor as u32 >> 18)) as usize
	};
}

macro_rules! opname {
	($offset:expr, $code:expr) => {
		OpCode::n($code[$offset])
			.expect("Not an instruction")
			.to_string()
	};
}

impl Chunk {
	pub fn new() -> Self {
		Self {
			code: vec![],
			constants: vec![],
		}
	}

	pub fn emit_byte(&mut self, opcode: OpCode) {
		self.code.push(opcode as u8);
	}

	fn add_constant(&mut self, value: Value) -> usize {
		self.constants.push(value);

		return self.constants.len() - 1;
	}

	pub fn emit_bytes(&mut self, bytes: &[u8]) {
		self.code.extend_from_slice(bytes);
	}

	pub fn write_constant(&mut self, value: Value) {
		let index = self.add_constant(value);

		if index > 255 {
			let minor = (index & 0xFF) as u8;
			let mid = ((index >> 8) & 0xFF) as u8;
			let major = ((index >> 16) & 0xFF) as u8;

			self.emit_bytes(&[
				OpCode::Wide as u8,
				OpCode::Constant as u8,
				minor,
				mid,
				major,
			]);
		} else {
			self.emit_bytes(&[OpCode::Constant as u8, index as u8]);
		}
	}

	// disassembler

	pub fn disassemble(&self) {
		let mut offset: usize = 0;
		let mut is_wide: bool = false;

		while offset < self.code.len() {
			is_wide = self.instruction(&mut offset, is_wide);
		}
	}

	pub fn instruction(&self, offset: &mut usize, is_wide: bool) -> bool {
		if *offset >= self.code.len() {
			return false;
		}

		if let Some(opcode) = OpCode::n(self.code[*offset]) {
			match opcode {
				OpCode::Constant => self.constant_instruction(offset, is_wide),
				OpCode::Wide => {
					self.simple_instruction(offset);
					return true;
				}

				_ => self.simple_instruction(offset),
			};
		} else {
			println!("{} UNKNOWN", *offset);
		}

		return false;
	}

	fn simple_instruction(&self, offset: &mut usize) {
		println!("{} {}", *offset, opname!(*offset, self.code));

		*offset += 1;
	}

	fn constant_instruction(&self, offset: &mut usize, is_wide: bool) {
		let jump: usize;
		let index: usize;

		if is_wide {
			jump = 4;
			index = from_u24!(
				self.code[*offset + 3],
				self.code[*offset + 2],
				self.code[*offset + 1]
			);
		} else {
			jump = 2;
			index = self.code[*offset + 1] as usize;
		}

		println!(
			"{} {} {} ({})",
			*offset,
			opname!(*offset, self.code),
			index,
			self.constants[index]
		);

		*offset += jump;
	}
}
