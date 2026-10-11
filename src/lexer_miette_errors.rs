use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Unexpected character '{character}'")]
#[diagnostic(help("Remove it"))]
pub struct UnknownCharacter {
    #[source_code]
    pub source_code: String,

    pub character: char,

    #[label = "this character"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Unterminated string")]
#[diagnostic(help("Close the string with \""))]
pub struct UnterminatedString {
    #[source_code]
    pub source_code: String,

    #[label = "this string literal"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Malformed number")]
#[diagnostic(help("Only floating point numbers are supported"))]
pub struct MalformedNumber {
    #[source_code]
    pub source_code: String,

    #[label = "this number"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Unterminated block comment")]
#[diagnostic(help("Close the comment with */"))]
pub struct UnterminatedBlockComment {
    #[source_code]
    pub source_code: String,

    #[label = "this block comment"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Malformed number")]
#[diagnostic(help("Only floating point numbers are supported"))]
pub struct ExpressionParseError {
    #[source_code]
    pub source_code: String,
}
