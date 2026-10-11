use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Bad binary operator")]
#[diagnostic(help("Only + is supported"))]
pub struct UnexpectedBinaryOperator {
    #[source_code]
    pub source_code: String,

    #[label = "this operator"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Unexpected literal")]
#[diagnostic(help("idk"))]
pub struct UnexpectedLiteral {
    #[source_code]
    pub source_code: String,

    #[label = "this operator"]
    pub err_span: SourceSpan,
}


#[derive(Diagnostic, Debug, Error)]
#[error("Unclosed parenthesis")]
#[diagnostic(help("Close parenthesis"))]
pub struct UnclosedParenthesis {
    #[source_code]
    pub source_code: String,

    #[label = "this operator"]
    pub err_span: SourceSpan,
}
