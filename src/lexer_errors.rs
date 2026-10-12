use std::ops::Range;

use crate::lexer_miette_errors;

pub enum LexerError {
    UnterminatedString { span: Range<usize> },
    UnterminatedBlockComment { span: Range<usize> },
    MalformedNumber { span: Range<usize> },
    UnknownCharacter { span: Range<usize>, character: char },
}

impl LexerError {
    pub fn into_miette_report(self, source_code: &str) -> miette::Report {
        match self {
            LexerError::UnterminatedString { span } => lexer_miette_errors::UnterminatedString {
                source_code: source_code.into(),
                err_span: span.into(),
            }
            .into(),
            LexerError::UnterminatedBlockComment { span } => {
                lexer_miette_errors::UnterminatedBlockComment {
                    source_code: source_code.into(),
                    err_span: span.into(),
                }
                .into()
            }
            LexerError::MalformedNumber { span } => lexer_miette_errors::MalformedNumber {
                source_code: source_code.into(),
                err_span: span.into(),
            }
            .into(),
            LexerError::UnknownCharacter { span, character } => {
                lexer_miette_errors::UnknownCharacter {
                    source_code: source_code.into(),
                    err_span: span.into(),
                    character,
                }
                .into()
            }
        }
    }
}
