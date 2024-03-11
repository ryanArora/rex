use {clap::Parser, std::path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[arg()]
    input_file: PathBuf,
    #[arg(short)]
    output_file: Option<PathBuf>,
}

fn main() {
    let _cli = Cli::parse();
    unimplemented!();
}
