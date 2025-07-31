use bignums::error::ParseFloatError;
use bignums::real::Real;
use crate::parser::{BinOp, Expr, UnaryOp};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EvalError {
	ArithmeticError,
	ParseFloatError(ParseFloatError),
	InvalidConst(String),
	InvalidFnCall(String),
}

impl std::fmt::Display for EvalError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			EvalError::ArithmeticError => write!(f, "Arithmetic error"),
			EvalError::ParseFloatError(e) => write!(f, "Parse float error: {}", e),
			EvalError::InvalidConst(s) => write!(f, "Invalid constant: {}", s),
			EvalError::InvalidFnCall(s) => write!(f, "Invalid function call: {}", s),
		}
	}
}

impl<'a> Expr<'a> {
	pub fn eval(&self, tol: i64) -> Result<Real, EvalError> {
		std::panic::catch_unwind(|| self.eval_internal(tol)).map_err(|_| EvalError::ArithmeticError)?
	}
	
	fn eval_internal(&self, tol: i64) -> Result<Real, EvalError> {
		match self {
			Expr::Number(s) => {
				Ok(Real::from_string(s.to_string()).map_err(|e| EvalError::ParseFloatError(e))?)
			}
			Expr::Const(s) => {
				match *s {
					"pi" => Ok(Real::pi()),
					_ => Err(EvalError::InvalidConst(s.to_string()))
				}
			}
			Expr::FnCall { name, args } => {
				match *name {
					"ln" => Ok(args[0].eval_internal(tol)?.ln(tol).map_err(|_| EvalError::ArithmeticError)?),
					"sqrt" => Ok(args[0].eval_internal(tol)?.sqrt()),
					_ => Err(EvalError::InvalidFnCall(name.to_string()))
				}
			}
			Expr::Binary { op, left, right } => {
				let l = left.eval_internal(tol)?;
				let r = right.eval_internal(tol)?;
				match *op {
					BinOp::Add => { Ok(l + r) }
					BinOp::Sub => { Ok(l - r) }
					BinOp::Mul => { Ok(l * r) }
					BinOp::Div => { Ok(l.div(r, tol).map_err(|_| EvalError::ArithmeticError)?) }
				}
			}
			Expr::Unary { op, operand } => {
				let arg = operand.eval_internal(tol)?;
				match *op {
					UnaryOp::Neg => { Ok(-arg) }
					UnaryOp::Pos => { Ok(arg) }
				}
			}
		}
	}
}