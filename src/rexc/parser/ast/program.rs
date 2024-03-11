use crate::parser::{syntax_error::SyntaxError, Parser};

#[derive(Debug, Clone)]
pub struct Program {}

impl Parser<'_> {
	pub fn next_program(&mut self) -> Result<Option<Program>, SyntaxError> {
		while let Some(token) = self.lexer.next_token() {
			println!("{:?}", token);
		}

		Ok(Some(Program {}))
	}
}
