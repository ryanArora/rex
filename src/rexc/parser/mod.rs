mod ast;
mod lexer;
mod syntax_error;

use {
	ast::program::Program,
	lexer::Lexer,
	std::{fs::File, io::BufReader, path::PathBuf},
	syntax_error::SyntaxError,
	utf8_chars::BufReadCharsExt,
};

struct Parser<'a> {
	lexer: Lexer<'a>,
}

pub fn get_program(source_filename: PathBuf) -> Result<Option<Program>, SyntaxError> {
	let source_file = File::open(source_filename).unwrap();
	let mut reader = BufReader::new(source_file);

	let lexer = Lexer {
		chars: reader.chars(),
	};

	let mut parser = Parser { lexer };
	parser.next_program()
}
