use clap::Parser;
use compiler::compile_file;
use std::path::PathBuf;

mod backend;
mod compiler;
mod error;
mod visitor;

#[derive(Parser)]
struct Args {
    input: PathBuf,
    #[arg(long, short)]
    output: Option<PathBuf>,
}

fn main() -> Result<(), error::Error> {
    let args = Args::parse();
    compile_file(&args.input, &args.output.unwrap_or("a.out".into()))?;

    Ok(())
}
