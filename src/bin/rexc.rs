use {clap::Parser, std::path::PathBuf};

#[derive(Debug, Parser)]
struct Cli {
    #[arg()]
    input_file: Vec<PathBuf>,
    #[arg(short, default_value = "a.out")]
    output_file: PathBuf,
}

fn main() {
    let _cli = Cli::parse();
}
