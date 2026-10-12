use std::ops::Range;

use crate::runtime_miette_errors;

#[derive(Debug, thiserror::Error)]
pub enum RuntimeErrorKind {
    #[error("Invalid operands for binary operator")]
    BinaryApplicationError,

    #[error("Invalid operand for unary operator")]
    UnaryApplicationError,

    #[error("Invalid ternary condition")]
    TernaryConditionalError,
}

pub struct RuntimeError {
    pub kind: RuntimeErrorKind,
    pub span: Range<usize>,
}

impl RuntimeError {
    pub fn into_miette_report(self, source_code: &str) -> miette::Report {
        match self.kind {
            RuntimeErrorKind::BinaryApplicationError => {
                runtime_miette_errors::BinaryApplicationError {
                    source_code: source_code.into(),
                    err_span: self.span.into(),
                }
                .into()
            }
            RuntimeErrorKind::UnaryApplicationError => {
                runtime_miette_errors::UnaryApplicationError {
                    source_code: source_code.into(),
                    err_span: self.span.into(),
                }
                .into()
            }
            RuntimeErrorKind::TernaryConditionalError => {
                runtime_miette_errors::TernaryApplicationError {
                    source_code: source_code.into(),
                    err_span: self.span.into(),
                }
                .into()
            }
        }
    }
}
