use crate::{backend::X64Backend, error, visitor::NodeVisitor};
use oxc_allocator::Allocator;
use oxc_ast::{visit::Visit, SourceType};
use oxc_parser::Parser;
use std::{fs::read_to_string, path::Path};

pub fn compile_file(path: &Path) -> Result<(), error::Error> {
    let source = read_to_string(path)?;

    let source_type = SourceType::from_path(path).unwrap();
    let allocator = Allocator::default();
    let parser = Parser::new(&allocator, &source, source_type).parse();

    if !parser.errors.is_empty() {
        // TODO: Make a custom diagnostic printer... this works for now
        for error in parser.errors {
            let error = error.with_source_code(source.clone());
            println!("{error:?}");
        }

        // TODO: We should probably return with an Err type
        return Ok(());
    }

    let program = allocator.alloc(parser.program);

    let backend = X64Backend::new()?;
    let mut visitor = NodeVisitor::new(backend);
    visitor.visit_program(program);

    Ok(())
}
