use oxc_ast::ast::{Program, Statement, BlockStatement, Declaration, Function, VariableDeclaration, VariableDeclarator};

pub struct Context {
}

pub trait Gen {
    fn gen(&self, ctx: &mut Context);
}

impl Gen for Program<'_> {
    fn gen(&self, ctx: &mut Context) {
        for stmt in &self.body {
            visit_top_level_statement(ctx, stmt);
        }
    }
}

fn visit_top_level_statement(ctx: &mut Context, stmt: &Statement<'_>) {
    match stmt {
        Statement::Declaration(decl) => decl.gen(ctx),
        _ => panic!("TODO: Generate error diagnostic for unsupported top-level statement.")
    }
}

impl Gen for Statement<'_> {
    fn gen(&self, ctx: &mut Context) {
        match self {
            Statement::BlockStatement(stmt) => stmt.gen(ctx),
            Statement::Declaration(decl) => decl.gen(ctx),
            _ => {}
        }
    }
}

impl Gen for BlockStatement<'_> {
    fn gen(&self, ctx: &mut Context) {
        for stmt in &self.body {
            println!("{:?}", stmt);
            stmt.gen(ctx);
        }
    }
}

impl Gen for Declaration<'_> {
    fn gen(&self, ctx: &mut Context) {
        match self {
            Declaration::FunctionDeclaration(func) => func.gen(ctx),
            Declaration::VariableDeclaration(var) => var.gen(ctx),
            _ => {}
        }
    }
}

impl Gen for Function<'_> {
    fn gen(&self, ctx: &mut Context) {
        println!("Function {:?} (args: {})", self.id, self.params.items.len());

        if let Some(body) = &self.body {
            for stmt in &body.statements {
                stmt.gen(ctx);
            }
        }
    }
}

impl Gen for VariableDeclaration<'_> {
    fn gen(&self, ctx: &mut Context) {
        for decl in &self.declarations {
            decl.gen(ctx);
        }
    }
}

impl Gen for VariableDeclarator<'_> {
    fn gen(&self, ctx: &mut Context) {
        println!("VariableDeclarator {:?} = {:?}", self.id, self.init.as_ref().unwrap());
    }
}