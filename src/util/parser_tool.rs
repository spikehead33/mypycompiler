use rustpython_parser::ast::{Expr, ExprName, ExprTuple, Mod, Stmt};

use super::{CompilerError, CompilerResult};

pub fn extract_module_body(mod_: Mod) -> CompilerResult<Vec<Stmt>> {
    if let Mod::Module(m) = mod_ {
        Ok(m.body)
    } else {
        // TODO: make a better CompilerError
        Err(CompilerError::UnknownError)
    }
}

pub fn extract_variables(node: &Expr) -> Vec<ExprName> {
    let mut variables = vec![];
    match node {
        Expr::Tuple(ExprTuple { elts, .. }) => {
            for elt in elts.iter() {
                let mut vars = extract_variables(elt);
                variables.append(&mut vars);
            }
        }
        Expr::Name(name) => variables.push(name.clone()),
        _ => (),
    }

    variables
}

#[cfg(test)]
mod test {
    #[test]
    fn test_try() {
        assert_eq!(1, 1);
    }
}
