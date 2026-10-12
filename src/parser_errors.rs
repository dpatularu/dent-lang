use std::ops::Range;

use crate::parser_miette_errors;

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
                parser_miette_errors::UnexpectedBinaryOperator {
                    source_code: source_code.into(),
                    err_span: (span.start, span.len()).into(),
                }
                .into()
            }
            ParserError::UnexpectedUnaryOperator { span } => {
                parser_miette_errors::UnexpectedUnaryOperator {
                    source_code: source_code.into(),
                    err_span: (span.start, span.len()).into(),
                }
                .into()
            }
            ParserError::UnexpectedExpression { span } => {
                parser_miette_errors::UnexpectedExpression {
                    source_code: source_code.into(),
                    err_span: (span.start, span.len()).into(),
                }
                .into()
            }
            ParserError::UnclosedGroup { span } => parser_miette_errors::UnclosedParenthesis {
                source_code: source_code.into(),
                err_span: (span.start, span.len()).into(),
            }
            .into(),
            ParserError::UnexpectedEof => parser_miette_errors::UnexpectedEof {
                source_code: source_code.into(),
            }
            .into(),
        }
    }
}
