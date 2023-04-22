use crate::backend::Backend;
use oxc_ast::{visit::Visit, AstKind};

pub struct NodeVisitor<'a, B>
where
    B: Backend,
{
    backend: &'a mut B,
}

impl<B: Backend> NodeVisitor<'_, B> {
    pub fn new(backend: &mut B) -> NodeVisitor<B> {
        NodeVisitor { backend }
    }
}

impl<'a, B: Backend> Visit<'a> for NodeVisitor<'_, B> {
    fn enter_node(&mut self, kind: oxc_ast::AstKind<'a>) {
        let res = match kind {
            AstKind::Function(func) => self.backend.create_function(func),
            _ => Ok(()),
        };

        match res {
            Ok(()) => (),
            Err(err) => panic!("{err}"),
        }
    }

    fn leave_node(&mut self, kind: oxc_ast::AstKind<'a>) {
        let res = match kind {
            AstKind::Function(func) => self.backend.exit_function(func),
            _ => Ok(()),
        };

        match res {
            Ok(()) => (),
            Err(err) => panic!("{err}"),
        }
    }
}
