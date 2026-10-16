use std::ops::Range;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeErrorKind {
    #[error("Invalid operands for binary operator")]
    BinaryApplicationError,

    #[error("Invalid operand for unary operator")]
    UnaryApplicationError,

    #[error("Invalid ternary condition")]
    TernaryConditionalError,
}

#[derive(Debug)]
pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub span: Range<usize>,
}

impl RuntimeError {
    pub fn into_miette_report(self, source_code: &str) -> miette::Report {
        match self.kind {
            RuntimeErrorKind::BinaryApplicationError => {
                BinaryApplicationError {
                    source_code: source_code.into(),
                    err_span: self.span.into(),
                }
                .into()
            }
            RuntimeErrorKind::UnaryApplicationError => {
                UnaryApplicationError {
                    source_code: source_code.into(),
                    err_span: self.span.into(),
                }
                .into()
            }
            RuntimeErrorKind::TernaryConditionalError => {
                TernaryApplicationError {
                    source_code: source_code.into(),
                    err_span: self.span.into(),
                }
                .into()
            }
            RuntimeErrorKind::TernaryConditionalError => {
                TernaryApplicationError {
                    source_code: source_code.into(),
                    err_span: self.span.into(),
                }
                .into()
            }
        }
    }
}

use miette::{Diagnostic, SourceSpan};
use thiserror::Error;

#[derive(Diagnostic, Debug, Error)]
#[error("Bad binary")]
pub struct BinaryApplicationError {
    #[source_code]
    pub source_code: String,

    #[label = "this expression"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Bad unary")]
pub struct UnaryApplicationError {
    #[source_code]
    pub source_code: String,

    #[label = "this expression"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Bad ternary")]
pub struct TernaryApplicationError {
    #[source_code]
    pub source_code: String,

    #[label = "this expression"]
    pub err_span: SourceSpan,
}

#[derive(Diagnostic, Debug, Error)]
#[error("Unterminated string")]
pub struct UnterminatedString {
    #[source_code]
    pub source_code: String,

    #[label = "this string literal"]
    pub err_span: SourceSpan,
}
