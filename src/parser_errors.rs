use std::ops::Range;

use crate::parser_miette_errors;

pub enum ParserError {
    UnexpectedBinaryOperator { span: Range<usize> },
    UnexpectedUnaryOperator { span: Range<usize> },
    UnexpectedLiteral { span: Range<usize>},
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
                parser_miette_errors::UnexpectedBinaryOperator {
                    source_code: source_code.into(),
                    err_span: (span.start, span.len()).into(),
                }
                .into()
            }
                        ParserError::UnexpectedLiteral { span } => {
                parser_miette_errors::UnexpectedLiteral {
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
        }
    }
}
