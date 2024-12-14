mod grammar;
mod scanner;
mod utils;

pub use grammar::*;

pub struct Source<'a> {
    pub name: Option<&'a str>,
    pub content: &'a str,
}

pub type Result<Tok, T> = std::result::Result<T, ParseError<Tok>>;

//TODO: Improve ParseError
#[derive(Debug)]
pub enum ParseError<Tok> {
    ScanError { sample: String },
    UnexpectedToken { token: Tok },
    Eof,

    MissingImplementation,
}
