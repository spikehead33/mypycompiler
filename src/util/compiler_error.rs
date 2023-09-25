use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilerError {
    #[error("file operation error")]
    FileOperationError(#[from] io::Error),
    #[error("file parsing error")]
    ParserError(#[from] rustpython_parser::ParseError),
    #[error("unknown error")]
    UnknownError,
}
