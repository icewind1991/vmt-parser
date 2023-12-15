use miette::{Diagnostic, SourceSpan};
use thiserror::Error;
use vdf_reader::entry::Entry;
use vdf_reader::error::ParseEntryError;
use vdf_reader::VdfError;

#[derive(Debug, Error, Diagnostic)]
pub enum VmtError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Vdf(#[from] VdfError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    Eof(#[from] EofError),
    #[error(transparent)]
    #[diagnostic(transparent)]
    ParseValue(#[from] ParseValueError),
}
#[derive(Debug, Error, Diagnostic)]
#[error("Unexpected end of input while looking for {expected}")]
#[diagnostic(code(vmt_parser::eof))]
pub struct EofError {
    expected: &'static str,
    #[label("Expected {}", self.expected)]
    err_span: SourceSpan,
    #[source_code]
    src: String,
}

impl EofError {
    pub fn new(src: String, expected: &'static str) -> Self {
        let span = src.len()..src.len();
        EofError {
            src,
            err_span: span.into(),
            expected,
        }
    }
}
#[derive(Debug, Error, Diagnostic)]
#[error("Can't parse value {value:?} as {ty} to read {name}")]
#[diagnostic(code(vmt_parser::eof))]
pub struct ParseValueError {
    name: &'static str,
    ty: &'static str,
    pub value: Entry,
    #[label("Expected a {}", self.ty)]
    err_span: SourceSpan,
    #[source_code]
    src: String,
}

impl ParseValueError {
    pub fn new(
        src: String,
        name: &'static str,
        err_span: SourceSpan,
        parse_err: ParseEntryError,
    ) -> Self {
        ParseValueError {
            src,
            err_span,
            name,
            ty: parse_err.ty,
            value: parse_err.value,
        }
    }
}
