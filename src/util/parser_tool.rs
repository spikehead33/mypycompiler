use rustpython_parser::ast;

use super::{CompilerError, CompilerResult};

pub fn extract_module_body(mod_: ast::Mod) -> CompilerResult<Vec<ast::Stmt>> {
    if let ast::Mod::Module(m) = mod_ {
        Ok(m.body)
    } else {
        // TODO: make a better CompilerError
        Err(CompilerError::UnknownError)
    }
}
