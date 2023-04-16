use crate::error;
use iced_x86::code_asm::CodeAssembler;
use oxc_ast::ast::Function;

pub trait Backend {
    fn create_function(&mut self, func: &Function) -> Result<(), error::Error>;
}

pub struct X64Backend {
    asm: CodeAssembler,
}

impl X64Backend {
    pub fn new() -> Result<X64Backend, error::Error> {
        let asm = CodeAssembler::new(64)?;
        Ok(X64Backend { asm })
    }
}

impl Backend for X64Backend {
    fn create_function(&mut self, func: &Function) -> Result<(), error::Error> {
        // if !func.is_declaration() {
        //     return Ok(());
        // }

        // let name = if let Some(id) = &func.id {
        //     id.name.as_str()
        // } else {
        //     return Ok(());
        // };

        // let mut function_label = self.asm.create_label();
        // self.asm.set_label(&mut function_label);

        Ok(())
    }
}
