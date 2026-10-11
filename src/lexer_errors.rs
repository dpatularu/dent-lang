use std::ops::Range;

use crate::miette_errors;

pub enum LexerError {
    UnterminatedString { span: Range<usize> },
    UnterminatedBlockComment { span: Range<usize> },
    MalformedNumber { span: Range<usize> },
    UnknownCharacter { span: Range<usize>, character: char },
}

impl LexerError {
    pub fn into_miette_report(self, source_code: &str) -> miette::Report {
        match self {
            LexerError::UnterminatedString { span } => miette_errors::UnterminatedString {
                source_code: source_code.into(),
                err_span: (span.start, span.len()).into(),
            }
            .into(),
            LexerError::UnterminatedBlockComment { span } => {
                miette_errors::UnterminatedBlockComment {
                    source_code: source_code.into(),
                    err_span: (span.start, span.len()).into(),
                }
                .into()
            }
            LexerError::MalformedNumber { span } => miette_errors::MalformedNumber {
                source_code: source_code.into(),
                err_span: (span.start, span.len()).into(),
            }
            .into(),
            LexerError::UnknownCharacter { span, character } => miette_errors::UnknownCharacter {
                source_code: source_code.into(),
                err_span: (span.start, span.len()).into(),
                character,
            }
            .into(),
        }
    }
}
