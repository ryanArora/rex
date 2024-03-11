use {
	clap::{Parser, Subcommand},
	std::path::PathBuf,
};

#[derive(Parser)]
struct Cli {
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands {
	Run {
		#[arg(default_value = ".")]
		project_path: PathBuf,
	},
	Build {
		#[arg(default_value = ".")]
		project_path: PathBuf,
	},
}

fn main() {
	let _cli = Cli::parse();
	unimplemented!();
}
