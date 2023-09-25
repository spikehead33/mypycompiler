use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, module::Module, values::AnyValueEnum};
use rustpython_parser::ast::{
    Stmt, StmtAnnAssign, StmtClassDef, StmtExpr, StmtFor, StmtFunctionDef, StmtIf, StmtReturn,
    StmtWhile,
};

pub struct Codegen<'ctx, 'a> {
    pub context: &'ctx Context,
    pub module: &'a Module<'ctx>,
    pub builder: &'a Builder<'ctx>,
    pub environment: HashMap<String, AnyValueEnum<'ctx>>,
}

impl<'ctx, 'a> Codegen<'ctx, 'a> {
    pub fn gen_stmts(&self, stmts: &Vec<Stmt>) {
        for stmt in stmts.iter() {
            self.gen_stmt(stmt);
        }
    }

    pub fn gen_stmt(&self, stmt: &Stmt) {
        match stmt {
            Stmt::AnnAssign(x) => (),
            Stmt::FunctionDef(field) => (),
            Stmt::ClassDef(field) => (),
            Stmt::For(field) => (),
            Stmt::While(field) => (),
            Stmt::Expr(field) => (),
            Stmt::If(field) => (),
            Stmt::Return(field) => (),
            _ => (),
        }
    }
}
