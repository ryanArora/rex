mod parser;
use {clap::Parser, parser::get_program, std::path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
	#[arg()]
	input_file: PathBuf,
	#[arg(short)]
	output_file: Option<PathBuf>,
}

fn main() {
	let cli = Cli::parse();
	let _program = get_program(cli.input_file);
}
