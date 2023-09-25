mod codegen;
mod compiler;
mod util;

use util::CompilerResult;

fn main() -> CompilerResult<()> {
    // TODO: adding command line argument parsing
    let filename = "test.py";
    compiler::run(filename)
}
