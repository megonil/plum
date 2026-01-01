use std::fmt::Display;

use crate::error::{ValueError::*, ValueResult};

pub type PlumInt = i32;
pub type PlumFloat = f64;

#[derive(Clone, Copy)]
pub enum ObjType {
	String,
	Function,
	NativeFunction,
}

#[derive(Clone, Copy)]
pub struct Obj {
	kind: ObjType,
}

#[derive(Clone, Copy)]
pub(crate) enum Value {
	Int(i32),
	Num(f64),
}

impl Value {
	pub fn pow(self, rhs: Value) -> ValueResult<Value> {
		Ok(self.binop(rhs, |a, b| a.pow(b.try_into().unwrap()), |a, b| a.powf(b)))
	}

	fn binop<FInt, FFloat>(self, rhs: Value, int_op: FInt, float_op: FFloat) -> Value
	where
		FInt: FnOnce(PlumInt, PlumInt) -> PlumInt,
		FFloat: FnOnce(PlumFloat, PlumFloat) -> PlumFloat,
	{
		match (self, rhs) {
			(Value::Int(a), Value::Int(b)) => Value::Int(int_op(a, b)),
			(a, b) => {
				let af = match a {
					Value::Int(x) => x as PlumFloat,
					Value::Num(x) => x,
				};
				let bf = match b {
					Value::Int(x) => x as PlumFloat,
					Value::Num(x) => x,
				};

				Value::Num(float_op(af, bf))
			}
		}
	}
}

impl Display for Value {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Value::Int(a) => write!(f, "{}", a),
			Value::Num(a) => write!(f, "{}", a),
		}
	}
}

macro_rules! impl_binop {
    ($Trait:ident, $method:ident, $op:tt) => {
        impl std::ops::$Trait for Value {
            type Output = Value;

            fn $method(self, rhs: Self) -> Self::Output {
                self.binop(rhs, |a, b| a $op b, |x, y| x $op y)
            }
        }
    };
}

impl_binop!(Add, add, +);
impl_binop!(Sub, sub, -);
impl_binop!(Mul, mul, *);
impl_binop!(Rem, rem, %);

impl std::ops::Div for Value {
	type Output = ValueResult<Value>;

	fn div(self, rhs: Self) -> Self::Output {
		match (self, rhs) {
			(_, Value::Int(0)) => Err(FloatPointException()),
			(_, Value::Num(0.0)) => Err(FloatPointException()),

			(Value::Int(a), Value::Int(b)) => Ok(Value::Num(a as PlumFloat / b as PlumFloat)),
			(Value::Int(a), Value::Num(b)) => Ok(Value::Num(a as PlumFloat / b)),
			(Value::Num(a), Value::Num(b)) => Ok(Value::Num(a / b)),
			(Value::Num(a), Value::Int(b)) => Ok(Value::Num(a / b as PlumFloat)),
		}
	}
}
