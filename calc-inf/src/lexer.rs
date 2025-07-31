
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Lexer<'a>(pub &'a str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Token<'a> {
	Number(&'a str),
	Identifier(&'a str),
	Plus,
	Minus,
	Multiply,
	Divide,
	LParen,
	RParen,
	Comma,
	Error
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token<'a>;

	fn next(&mut self) -> Option<Self::Item> {
		let (c, rest) = loop {
			let (c, rest) = split_first_char(self.0)?;

			if c.is_whitespace() {
				self.0 = rest;
				continue;
			}

			break (c, rest);
		};

		if c == '+' {
			self.0 = rest;
			return Some(Token::Plus);
		}
		if c == '-' {
			self.0 = rest;
			return Some(Token::Minus);
		}
		if c == '*' {
			self.0 = rest;
			return Some(Token::Multiply);
		}
		if c == '/' {
			self.0 = rest;
			return Some(Token::Divide);
		}
		if c == '(' {
			self.0 = rest;
			return Some(Token::LParen);
		}
		if c == ')' {
			self.0 = rest;
			return Some(Token::RParen);
		}
		if c == ',' {
			self.0 = rest;
			return Some(Token::Comma);
		}

		if c.is_digit(10) || c == '.' {
			let mut iter = self.0.char_indices();
			loop {
				let Some((i, next)) = iter.next() else {
					let res = self.0;
					self.0 = "";
					return Some(Token::Number(res));
				};

				if next.is_digit(10) || next == '.' {
					continue;
				}
				let res = self.0.get(..i).unwrap();
				self.0 = self.0.get(i..).unwrap();
				return Some(Token::Number(res));
			}
		}

		if c.is_alphabetic() {
			let mut iter = self.0.char_indices();
			loop {
				let Some((i, next)) = iter.next() else {
					let res = self.0;
					self.0 = "";
					return Some(Token::Identifier(res));
				};

				if next.is_alphanumeric() {
					continue;
				}
				let res = self.0.get(..i).unwrap();
				self.0 = self.0.get(i..).unwrap();
				return Some(Token::Identifier(res));
			}
		}

		Some(Token::Error)
	}
}

fn split_first_char(s: &str) -> Option<(char, &str)> {
	let mut chars = s.chars();
	let first = chars.next()?;
	let rest = chars.as_str();
	Some((first, rest))
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_numbers() {
		let input = "123 45.67 .89";
		let mut lexer = Lexer(input);
		assert!(matches!(lexer.next(), Some(Token::Number("123"))));
		assert!(matches!(lexer.next(), Some(Token::Number("45.67"))));
		assert!(matches!(lexer.next(), Some(Token::Number(".89"))));
		assert!(matches!(lexer.next(), None));
	}

	#[test]
	fn test_identifiers() {
		let input = "abc x123 y";
		let mut lexer = Lexer(input);
		assert!(matches!(lexer.next(), Some(Token::Identifier("abc"))));
		assert!(matches!(lexer.next(), Some(Token::Identifier("x123"))));
		assert!(matches!(lexer.next(), Some(Token::Identifier("y"))));
		assert!(matches!(lexer.next(), None));
	}

	#[test]
	fn test_operators() {
		let input = "+-*/()";
		let mut lexer = Lexer(input);
		assert!(matches!(lexer.next(), Some(Token::Plus)));
		assert!(matches!(lexer.next(), Some(Token::Minus)));
		assert!(matches!(lexer.next(), Some(Token::Multiply)));
		assert!(matches!(lexer.next(), Some(Token::Divide)));
		assert!(matches!(lexer.next(), Some(Token::LParen)));
		assert!(matches!(lexer.next(), Some(Token::RParen)));
		assert!(matches!(lexer.next(), None));
	}

	#[test]
	fn test_mixed_expression() {
		let input = "2 * (x + 3.14)";
		let mut lexer = Lexer(input);
		assert!(matches!(lexer.next(), Some(Token::Number("2"))));
		assert!(matches!(lexer.next(), Some(Token::Multiply)));
		assert!(matches!(lexer.next(), Some(Token::LParen)));
		assert!(matches!(lexer.next(), Some(Token::Identifier("x"))));
		assert!(matches!(lexer.next(), Some(Token::Plus)));
		assert!(matches!(lexer.next(), Some(Token::Number("3.14"))));
		assert!(matches!(lexer.next(), Some(Token::RParen)));
		assert!(matches!(lexer.next(), None));
	}

	#[test]
	fn test_whitespace() {
		let input = "  123   abc  ";
		let mut lexer = Lexer(input);
		assert!(matches!(lexer.next(), Some(Token::Number("123"))));
		assert!(matches!(lexer.next(), Some(Token::Identifier("abc"))));
		assert!(matches!(lexer.next(), None));
	}
}
