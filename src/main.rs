use clap::Parser;
use oxc_allocator::Allocator;
use oxc_ast::SourceType;
use oxc_parser::Parser as OxcParser;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use codegen::{Gen, Context};

mod bytecode;
mod codegen;

#[derive(Parser)]
struct Args {
    input: PathBuf,
    #[arg(long, short)]
    output: Option<PathBuf>,
}

fn main() {
    let args = Args::parse();

    let mut sourcefile = match File::open(args.input.as_path()) {
        Ok(file) => file,
        Err(err) => panic!("{err}"),
    };
    let mut source = String::new();
    _ = sourcefile.read_to_string(&mut source);

    let allocator = Allocator::default();
    let source_type = SourceType::from_path(args.input.as_path()).unwrap();

    let ret = OxcParser::new(&allocator, &source, source_type).parse();
    if !ret.errors.is_empty() {
        for error in ret.errors {
            let error = error.with_source_code(source.clone());
            println!("{error:?}");
        }
        return;
    }

    let mut context = Context{};

    let program = allocator.alloc(ret.program);
    program.gen(&mut context);

    // let errors = Compiler::compile(program);

    // if !errors.is_empty() {
    //     for error in errors {
    //         let error = error;
    //         println!("{error:?}");
    //     }
    // }
}
