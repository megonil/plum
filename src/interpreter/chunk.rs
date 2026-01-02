use crate::error::{ChunkError, ChunkResult, DisasmError::*, DisasmResult};
use crate::value::Value;

use super::opcodes::OpCode;

pub type Line = usize;

#[derive(Clone)]
pub struct Chunk {
	pub code: Vec<u8>,
	pub constants: Vec<Value>,
	pub lines: Vec<Line>,
}

macro_rules! from_u24 {
	($major:expr, $mid:expr, $minor:expr) => {
		($major as u32 | ($mid as u32 >> 8) | ($minor as u32 >> 18)) as usize
	};
}

macro_rules! opname {
	($offset:expr, $code:expr) => {
		OpCode::n($code[$offset])
			.ok_or(UnknownOpCode($code[$offset].into()))?
			.to_string()
	};
}

impl Chunk {
	pub fn new() -> Self {
		Self {
			code: vec![],
			constants: vec![],
			lines: vec![0],
		}
	}

	#[inline(always)]
	pub fn emit_byte(&mut self, opcode: OpCode) {
		self.code.push(opcode as u8);
	}

	#[inline(always)]
	fn add_constant(&mut self, value: Value) -> usize {
		self.constants.push(value);

		return self.constants.len() - 1;
	}

	#[inline(always)]
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

	#[inline(always)]
	pub fn patch_code(&mut self, index: usize, value: u8) {
		self.code[index] = value;
	}

	#[inline(always)]
	pub fn is_wide(&self) -> bool {
		self.code[self.code.len() - 1] == OpCode::Wide as u8
	}

	#[inline(always)]
	pub fn emit_pop(&mut self) {
		self.emit_byte(OpCode::Pop);
	}

	#[inline(always)]
	pub fn emit_return(&mut self) {
		self.emit_byte(OpCode::Return);
	}

	#[inline(always)]
	/// starts an jump
	/// you will need to call a end_jump where you want to jump
	/// return pointer to the minor byte of the 2-byte argument
	pub fn start_jump(&mut self, jmp: OpCode) -> usize {
		self.emit_bytes(&[jmp as u8, 0xFF, 0xFF]);

		return self.code.len() - 1;
	}

	pub fn end_jump(&mut self, jmp_idx: usize) -> ChunkResult<()> {
		let next_instruction = jmp_idx + 1;
		let len = self.code.len();

		let offset = len - next_instruction;

		if offset > i16::MAX as usize {
			return Err(ChunkError::TooManyCodeToJump);
		}

		let val = offset as u16;
		let minor = ((val >> 8) & 0xFF) as u8;
		let major = (val & 0xFF) as u8;

		self.patch_code(jmp_idx - 1, major);
		self.patch_code(jmp_idx, minor);

		Ok(())
	}

	// disassembler
	pub fn disassemble(&self) -> DisasmResult<()> {
		let mut offset: usize = 0;
		let mut is_wide: bool = false;

		while offset < self.code.len() {
			is_wide = self.instruction(&mut offset, is_wide)?;
		}

		Ok(())
	}

	pub fn instruction(&self, offset: &mut usize, is_wide: bool) -> DisasmResult<bool> {
		if *offset >= self.code.len() {
			return Ok(false);
		}

		if let Some(opcode) = OpCode::n(self.code[*offset]) {
			match opcode {
				OpCode::Constant => self.constant_instruction(offset, is_wide)?,
				OpCode::Wide => {
					self.simple_instruction(offset)?;
					return Ok(true);
				}
				OpCode::Jmp | OpCode::Jmpf => self.i16_arg_instruction(offset)?,
				_ => self.simple_instruction(offset)?,
			};
		} else {
			println!("{:>4} UNKNOWN", *offset);
			*offset += 1;
		}

		return Ok(false);
	}

	fn i16_arg_instruction(&self, offset: &mut usize) -> DisasmResult<()> {
		println!(
			"{:>4} {} {}",
			*offset,
			opname!(*offset, self.code),
			(self.code[*offset + 1] as u16 | ((self.code[*offset + 2] as u16) << 8)) as i16
				+ *offset as i16
				+ 3
		);

		*offset += 3;

		Ok(())
	}

	fn simple_instruction(&self, offset: &mut usize) -> DisasmResult<()> {
		println!("{:>4} {}", *offset, opname!(*offset, self.code));

		*offset += 1;
		Ok(())
	}

	fn constant_instruction(&self, offset: &mut usize, is_wide: bool) -> DisasmResult<()> {
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
			"{:>4} {} {} ({})",
			*offset,
			opname!(*offset, self.code),
			index,
			self.constants[index]
		);

		*offset += jump;
		Ok(())
	}
}
