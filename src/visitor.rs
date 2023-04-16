use crate::backend::Backend;
use oxc_ast::{visit::Visit, AstKind};

pub struct NodeVisitor<B>
where
    B: Backend,
{
    backend: B,
}

impl<B: Backend> NodeVisitor<B> {
    pub fn new(backend: B) -> NodeVisitor<B> {
        NodeVisitor { backend }
    }
}

impl<'a, B: Backend> Visit<'a> for NodeVisitor<B> {
    fn enter_node(&mut self, _kind: oxc_ast::AstKind<'a>) {
        let res = match _kind {
            AstKind::Function(func) => self.backend.create_function(func),
            _ => Ok(()),
        };

        match res {
            Ok(()) => (),
            Err(err) => panic!("{err}"),
        }
    }

    fn leave_node(&mut self, _kind: oxc_ast::AstKind<'a>) {}
}
