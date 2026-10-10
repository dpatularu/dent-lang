use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

use crate::tokens::Token;

#[derive(Diagnostic, Debug, Error)]
#[error("Unexpected token '{bad_char}'")]
#[diagnostic(help("Remove it"))]
pub struct SingleTokenError {
    #[source_code]
    pub source_code: String,

    pub bad_char: char,

    #[label = "this character"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Unterminated string")]
#[diagnostic(help("Close the string with \""))]
pub struct StringTerminationError {
    #[source_code]
    pub source_code: String,

    #[label = "this string literal"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Malformed number")]
#[diagnostic(help("Only floating point numbers are supported"))]
pub struct MalformedNumberError {
    #[source_code]
    pub source_code: String,

    #[label = "this number"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Malformed number")]
#[diagnostic(help("Only floating point numbers are supported"))]
pub struct ExpressionParseError {
    #[source_code]
    pub source_code: String,
}
