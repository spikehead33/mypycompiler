use std::collections::HashMap;

use inkwell::{builder::Builder, context::Context, module::Module, values::AnyValueEnum};
use rustpython_parser::ast::{
    Stmt, StmtAnnAssign, StmtClassDef, StmtExpr, StmtFor, StmtFunctionDef, StmtIf, StmtReturn,
    StmtWhile,
};

use crate::util::parser_tool;

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

    fn gen_stmt(&self, stmt: &Stmt) {
        match stmt {
            Stmt::AnnAssign(x) => self.gen_ann_assign(x),
            Stmt::FunctionDef(x) => (),
            Stmt::ClassDef(x) => (),
            Stmt::For(x) => (),
            Stmt::While(x) => (),
            Stmt::Expr(x) => (),
            Stmt::If(x) => (),
            Stmt::Return(x) => (),
            _ => (),
        }
    }

    fn gen_ann_assign(&self, node: &StmtAnnAssign) {
        let annotation = &node.annotation;
        let value = &node.value;
        let target = &node.target;
        // This isn't useful for me
        // let simple = &node.simple;
        // This only for debug purpose?
        // let range = &node.range;

        let variables = parser_tool::extract_variables(target);
    }
}
