pub mod common;
mod compiler_error;
pub mod parser_tool;

pub use compiler_error::CompilerError;
pub type CompilerResult<T> = Result<T, CompilerError>;
