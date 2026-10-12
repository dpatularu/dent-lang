use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Bad binary operator")]
#[diagnostic(help("Only +, -, * and / are supported"))]
pub struct UnexpectedBinaryOperator {
    #[source_code]
    pub source_code: String,

    #[label = "this operator"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Bad unary operator")]
#[diagnostic(help("Only - and ! are supported"))]
pub struct UnexpectedUnaryOperator {
    #[source_code]
    pub source_code: String,

    #[label = "this operator"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Unexpected expression")]
#[diagnostic(help("idk"))]
pub struct UnexpectedExpression {
    #[source_code]
    pub source_code: String,

    #[label = "this expression"]
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

#[derive(Diagnostic, Debug, Error)]
#[error("Unexpected Eof")]
pub struct UnexpectedEof {
    #[source_code]
    pub source_code: String,
}
