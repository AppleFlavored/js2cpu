use crate::error;
use iced_x86::code_asm::*;
use object::{write::{Object, Symbol, StandardSection, SymbolSection}, SymbolKind, SymbolScope, SymbolFlags};
use oxc_ast::ast::Function;
use std::io::{BufWriter, Write};

pub trait Backend {
    fn create_function(&mut self, func: &Function) -> Result<(), error::Error>;
    fn exit_function(&mut self, func: &Function) -> Result<(), error::Error>;
}

pub struct X64Backend<'a> {
    asm: CodeAssembler,
    obj: Object<'a>,
}

impl<'a> X64Backend<'a> {
    pub fn new() -> Result<X64Backend<'a>, error::Error> {
        let asm = CodeAssembler::new(64)?;
        let obj = Object::new(
            object::BinaryFormat::Elf,
            object::Architecture::X86_64,
            object::Endianness::Little,
        );
        Ok(X64Backend { asm, obj })
    }

    pub fn write<W: Write>(&self, writer: BufWriter<W>) -> Result<(), error::Error> {
        match self.obj.write_stream(writer) {
            Ok(_) => Ok(()),
            Err(err) => panic!("{err}"),
        }
    }
}

impl<'a> Backend for X64Backend<'a> {
    fn create_function(&mut self, func: &Function) -> Result<(), error::Error> {
        if !func.is_declaration() {
            return Ok(());
        }

        self.asm.push(rbp)?;
        self.asm.mov(rbp, rsp)?;

        // TODO: We are not generating instructions from statements at the moment.
        self.asm.nop()?;

        Ok(())
    }

    fn exit_function(&mut self, func: &Function) -> Result<(), error::Error> {
        self.asm.pop(rbp)?;
        self.asm.ret()?;

        let encoded = self.asm.assemble(0x00)?;
        self.asm.reset();

        let name = if let Some(id) = &func.id {
            id.name.as_str()
        } else {
            return Ok(());
        };

        let section_id = self.obj.section_id(StandardSection::Text);
        let symbol_id = self.obj.add_symbol(Symbol {
            name: name.into(),
            value: 0,
            size: encoded.len() as u64,
            kind: SymbolKind::Text,
            scope: SymbolScope::Compilation,
            weak: false,
            section: SymbolSection::Section(section_id),
            flags: SymbolFlags::None,
        });
        self.obj.add_symbol_data(symbol_id, section_id, &encoded, 1);

        Ok(())
    }
}
