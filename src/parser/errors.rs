use std::ops::Range;

pub enum ParserError {
    UnexpectedBinaryOperator { span: Range<usize> },
    UnexpectedUnaryOperator { span: Range<usize> },
    UnexpectedExpression { span: Range<usize> },
    UnexpectedEof,
    UnclosedGroup { span: Range<usize> },
}

impl ParserError {
    pub fn into_miette_report(self, source_code: &str) -> miette::Report {
        match self {
            ParserError::UnexpectedBinaryOperator { span } => {
                UnexpectedBinaryOperator {
                    source_code: source_code.into(),
                    err_span: (span.start, span.len()).into(),
                }
                .into()
            }
            ParserError::UnexpectedUnaryOperator { span } => {
                UnexpectedUnaryOperator {
                    source_code: source_code.into(),
                    err_span: (span.start, span.len()).into(),
                }
                .into()
            }
            ParserError::UnexpectedExpression { span } => {
                UnexpectedExpression {
                    source_code: source_code.into(),
                    err_span: (span.start, span.len()).into(),
                }
                .into()
            }
            ParserError::UnclosedGroup { span } => UnclosedParenthesis {
                source_code: source_code.into(),
                err_span: (span.start, span.len()).into(),
            }
            .into(),
            ParserError::UnexpectedEof => UnexpectedEof {
                source_code: source_code.into(),
            }
            .into(),
        }
    }
}

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

    #[label = "this expresion"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Unexpected Eof")]
pub struct UnexpectedEof {
    #[source_code]
    pub source_code: String,
}
