use miette::Diagnostic;
use thiserror::Error;
use vdf_reader::VdfError;

#[derive(Debug, Error, Diagnostic)]
pub enum VmtError {
    #[error(transparent)]
    #[diagnostic(transparent)]
    Vdf(#[from] VdfError),
}
