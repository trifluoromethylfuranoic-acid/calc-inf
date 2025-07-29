pub fn lex(input: &str) -> Lexer {
	Lexer(input)
}

pub struct Lexer<'a>(&'a str);

pub enum Token {
	Number(),
	Plus,
	Minus,
	Multiply,
	Divide,
	LParen,
	RParen,
}

impl<'a> Iterator for Lexer<'a> {
	type Item = Token;

	fn next(&mut self) -> Option<Self::Item> {
		todo!()
	}
}
