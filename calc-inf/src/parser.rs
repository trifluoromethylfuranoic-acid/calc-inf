use std::fmt;
use crate::lexer::Token;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'a> {
	Number(&'a str),
	Const(&'a str),
	FnCall {
		name: &'a str,
		args: Vec<Expr<'a>>,
	},
	Binary {
		op: BinOp,
		left: Box<Expr<'a>>,
		right: Box<Expr<'a>>,
	},
	Unary {
		op: UnaryOp,
		operand: Box<Expr<'a>>,
	},
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinOp {
	Add,
	Sub,
	Mul,
	Div,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOp {
	Neg,
	Pos,
}

#[derive(Debug)]
pub enum ParseError {
	UnexpectedToken(String),
	UnexpectedEof,
	InvalidExpression,
}

impl fmt::Display for ParseError {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ParseError::UnexpectedToken(token) => write!(f, "Unexpected token: {}", token),
			ParseError::UnexpectedEof => write!(f, "Unexpected end of input"),
			ParseError::InvalidExpression => write!(f, "Invalid expression"),
		}
	}
}

pub struct Parser<'a, 'b> {
	tokens: &'b [Token<'a>],
	current: usize,
}

impl<'a, 'b> Parser<'a, 'b> {
	pub fn new(tokens: &'b [Token<'a>]) -> Self {
		Self { tokens, current: 0 }
	}

	pub fn parse(&mut self) -> Result<Expr<'a>, ParseError> {
		let expr = self.parse_expression()?;
		if !self.is_at_end() {
			return Err(ParseError::UnexpectedToken(format!("{:?}", self.peek())));
		}
		Ok(expr)
	}

	// Parse expressions with precedence (lowest to highest):
	// Addition/Subtraction -> Multiplication/Division -> Unary -> Primary
	fn parse_expression(&mut self) -> Result<Expr<'a>, ParseError> {
		self.parse_addition()
	}

	fn parse_addition(&mut self) -> Result<Expr<'a>, ParseError> {
		let mut expr = self.parse_multiplication()?;

		while self.match_tokens(&[Token::Plus, Token::Minus]) {
			let op = match self.previous() {
				Token::Plus => BinOp::Add,
				Token::Minus => BinOp::Sub,
				_ => unreachable!(),
			};
			let right = self.parse_multiplication()?;
			expr = Expr::Binary {
				op,
				left: Box::new(expr),
				right: Box::new(right),
			};
		}

		Ok(expr)
	}

	fn parse_multiplication(&mut self) -> Result<Expr<'a>, ParseError> {
		let mut expr = self.parse_unary()?;

		while self.match_tokens(&[Token::Multiply, Token::Divide]) {
			let op = match self.previous() {
				Token::Multiply => BinOp::Mul,
				Token::Divide => BinOp::Div,
				_ => unreachable!(),
			};
			let right = self.parse_unary()?;
			expr = Expr::Binary {
				op,
				left: Box::new(expr),
				right: Box::new(right),
			};
		}

		Ok(expr)
	}

	fn parse_unary(&mut self) -> Result<Expr<'a>, ParseError> {
		if self.match_tokens(&[Token::Minus, Token::Plus]) {
			let op = match self.previous() {
				Token::Minus => UnaryOp::Neg,
				Token::Plus => UnaryOp::Pos,
				_ => unreachable!(),
			};
			let operand = self.parse_unary()?;
			return Ok(Expr::Unary {
				op,
				operand: Box::new(operand),
			});
		}

		self.parse_primary()
	}

	fn parse_primary(&mut self) -> Result<Expr<'a>, ParseError> {
		match self.peek() {
			Token::Number(n) => {
				self.advance();
				Ok(Expr::Number(n))
			}
			Token::Identifier(id) => {
				let name = id;
				self.advance();

				// Check if this is a function call (followed by '(')
				if self.check(&Token::LParen) {
					self.advance(); // consume '('
					let mut args = Vec::new();

					// Parse arguments if not empty
					if !self.check(&Token::RParen) {
						loop {
							args.push(self.parse_expression()?);
							if !self.match_tokens(&[Token::Comma]) {
								break;
							}
						}
					}

					if !self.check(&Token::RParen) {
						return Err(ParseError::UnexpectedToken("Expected ')'".to_string()));
					}
					self.advance(); // consume ')'

					Ok(Expr::FnCall { name, args })
				} else {
					// It's a constant
					Ok(Expr::Const(name))
				}
			}
			Token::LParen => {
				self.advance(); // consume '('
				let expr = self.parse_expression()?;
				if !self.check(&Token::RParen) {
					return Err(ParseError::UnexpectedToken("Expected ')'".to_string()));
				}
				self.advance(); // consume ')'
				Ok(expr)
			}
			Token::Error => Err(ParseError::InvalidExpression),
			_ => Err(ParseError::UnexpectedToken(format!("{:?}", self.peek()))),
		}
	}

	// Helper methods
	fn match_tokens(&mut self, tokens: &[Token]) -> bool {
		for token in tokens {
			if self.check(token) {
				self.advance();
				return true;
			}
		}
		false
	}

	fn check(&self, token: &Token) -> bool {
		if self.is_at_end() {
			false
		} else {
			std::mem::discriminant(&self.peek()) == std::mem::discriminant(token)
		}
	}

	fn advance(&mut self) -> Token<'a> {
		if !self.is_at_end() {
			self.current += 1;
		}
		self.previous()
	}

	fn is_at_end(&self) -> bool {
		self.current >= self.tokens.len()
	}

	fn peek(&self) -> Token<'a> {
		if self.is_at_end() {
			Token::Error // Use Error as EOF marker
		} else {
			self.tokens[self.current]
		}
	}

	fn previous(&self) -> Token<'a> {
		self.tokens[self.current - 1]
	}
}

// Pretty printing for the AST
impl<'a> fmt::Display for Expr<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Expr::Number(n) => write!(f, "{}", n),
			Expr::Const(name) => write!(f, "{}", name),
			Expr::FnCall { name, args } => {
				write!(f, "{}(", name)?;
				for (i, arg) in args.iter().enumerate() {
					if i > 0 {
						write!(f, ", ")?;
					}
					write!(f, "{}", arg)?;
				}
				write!(f, ")")
			}
			Expr::Binary { op, left, right } => {
				write!(f, "({} {} {})", left, op, right)
			}
			Expr::Unary { op, operand } => {
				write!(f, "({}{})", op, operand)
			}
		}
	}
}

impl fmt::Display for BinOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			BinOp::Add => write!(f, "+"),
			BinOp::Sub => write!(f, "-"),
			BinOp::Mul => write!(f, "*"),
			BinOp::Div => write!(f, "/"),
		}
	}
}

impl fmt::Display for UnaryOp {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			UnaryOp::Neg => write!(f, "-"),
			UnaryOp::Pos => write!(f, "+"),
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::lexer::Lexer;
	use super::*;

	#[test]
	fn test_simple_number() {
		let tokens = vec![Token::Number("42")];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();
		assert_eq!(result, Expr::Number("42"));
	}

	#[test]
	fn test_simple_addition() {
		let tokens = vec![
			Token::Number("1"),
			Token::Plus,
			Token::Number("2"),
		];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();

		if let Expr::Binary { op, left, right } = result {
			assert_eq!(op, BinOp::Add);
			assert_eq!(*left, Expr::Number("1"));
			assert_eq!(*right, Expr::Number("2"));
		} else {
			panic!("Expected binary expression");
		}
	}

	#[test]
	fn test_repeated_addition() {
		let tokens: Vec<_> = Lexer("2+5+6").collect();
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();
		
		println!("Parsed: {}", result);
		assert!(is_addition(&result));
	}
	
	fn is_addition(expr: &Expr) -> bool {
		if let Expr::Number(_) = expr { return true; }
		if let Expr::Const(_) = expr { return true; }
		if let Expr::Binary { op, left, right } = expr {
			if *op != BinOp::Add { return false; }
			is_addition(left) && is_addition(right)
		} else {
			false
		}
	}

	#[test]
	fn test_precedence() {
		// 2 + 3 * 4 should parse as 2 + (3 * 4)
		let tokens = vec![
			Token::Number("2"),
			Token::Plus,
			Token::Number("3"),
			Token::Multiply,
			Token::Number("4"),
		];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();

		if let Expr::Binary { op: BinOp::Add, left, right } = result {
			assert_eq!(*left, Expr::Number("2"));
			if let Expr::Binary { op: BinOp::Mul, left: mul_left, right: mul_right } = *right {
				assert_eq!(*mul_left, Expr::Number("3"));
				assert_eq!(*mul_right, Expr::Number("4"));
			} else {
				panic!("Expected multiplication on right side");
			}
		} else {
			panic!("Expected addition at top level");
		}
	}

	#[test]
	fn test_parentheses() {
		// (2 + 3) * 4
		let tokens = vec![
			Token::LParen,
			Token::Number("2"),
			Token::Plus,
			Token::Number("3"),
			Token::RParen,
			Token::Multiply,
			Token::Number("4"),
		];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();
		println!("Parsed: {}", result);
	}

	#[test]
	fn test_unary_minus() {
		// -42
		let tokens = vec![Token::Minus, Token::Number("42")];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();

		if let Expr::Unary { op, operand } = result {
			assert_eq!(op, UnaryOp::Neg);
			assert_eq!(*operand, Expr::Number("42"));
		} else {
			panic!("Expected unary expression");
		}
	}

	#[test]
	fn test_const() {
		let tokens = vec![Token::Identifier("PI")];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();
		assert_eq!(result, Expr::Const("PI"));
	}

	#[test]
	fn test_function_call_no_args() {
		let tokens = vec![
			Token::Identifier("rand"),
			Token::LParen,
			Token::RParen,
		];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();

		if let Expr::FnCall { name, args } = result {
			assert_eq!(name, "rand");
			assert_eq!(args.len(), 0);
		} else {
			panic!("Expected function call");
		}
	}

	#[test]
	fn test_function_call_with_args() {
		let tokens = vec![
			Token::Identifier("max"),
			Token::LParen,
			Token::Number("1"),
			Token::Comma,
			Token::Number("2"),
			Token::RParen,
		];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();

		if let Expr::FnCall { name, args } = result {
			assert_eq!(name, "max");
			assert_eq!(args.len(), 2);
			assert_eq!(args[0], Expr::Number("1"));
			assert_eq!(args[1], Expr::Number("2"));
		} else {
			panic!("Expected function call");
		}
	}

	#[test]
	fn test_nested_function_calls() {
		// sin(cos(x))
		let tokens = vec![
			Token::Identifier("sin"),
			Token::LParen,
			Token::Identifier("cos"),
			Token::LParen,
			Token::Identifier("x"),
			Token::RParen,
			Token::RParen,
		];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();
		println!("Nested function call: {}", result);
	}

	#[test]
	fn test_const_vs_function() {
		// PI + sin(x)
		let tokens = vec![
			Token::Identifier("PI"),
			Token::Plus,
			Token::Identifier("sin"),
			Token::LParen,
			Token::Identifier("x"),
			Token::RParen,
		];
		let mut parser = Parser::new(&tokens);
		let result = parser.parse().unwrap();

		if let Expr::Binary { op: BinOp::Add, left, right } = result {
			assert_eq!(*left, Expr::Const("PI"));
			if let Expr::FnCall { name, args } = *right {
				assert_eq!(name, "sin");
				assert_eq!(args.len(), 1);
				assert_eq!(args[0], Expr::Const("x"));
			} else {
				panic!("Expected function call on right side");
			}
		} else {
			panic!("Expected addition at top level");
		}
	}
}