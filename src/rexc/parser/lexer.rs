use {
	std::{fs::File, io::BufReader},
	utf8_chars::Chars,
};

#[derive(Debug, PartialEq, Eq)]
pub enum Token {
	Char(char),
}

pub struct Lexer<'a> {
	pub chars: Chars<'a, BufReader<File>>,
}

impl Lexer<'_> {
	pub fn next_token(&mut self) -> Option<Token> {
		match self.chars.next() {
			Some(ch) => Some(Token::Char(ch.unwrap())),
			None => None,
		}
	}
}
