use crate::{codegen::Codegen, util::common, util::parser_tool, CompilerResult};
use inkwell::context::Context;
use rustpython_parser::{parse, Mode};
use std::{collections::HashMap, fs};

pub fn run(filename: &str) -> CompilerResult<()> {
    let module_name = common::module_name(filename);

    let source = fs::read_to_string(filename)?;
    let program = parse(source.as_str(), Mode::Module, filename)?;
    let stmts = parser_tool::extract_module_body(program)?;

    println!("{:?}", stmts);

    let context = Context::create();
    let module = context.create_module(module_name);
    let builder = context.create_builder();
    let codegen = Codegen {
        context: &context,
        module: &module,
        builder: &builder,
        environment: HashMap::new(),
    };

    codegen.gen_stmts(stmts.as_ref());
    Ok(())
}
