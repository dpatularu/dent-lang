// use std::{iter::Peekable, vec::IntoIter};
//
// use miette::MietteError;
//
// use crate::{
//     expressions::Expression,
//     tokens::{
//         Token,
//         TokenKind::{self},
//     },
// };
//
// pub struct Parser<'a> {
//     tokens: Peekable<IntoIter<Token>>,
//     source_code: &'a str,
//     prev: Option<Token>,
// }
//
// impl<'a> Parser<'a> {
//     fn new(tokens: Vec<Token>, source_code: &'a str) -> Self {
//         Self {
//             tokens: tokens.into_iter().peekable(),
//             source_code,
//             prev: None,
//         }
//     }
//
//     fn next(&mut self) -> Option<Token> {
//         self.prev = self.tokens.next();
//         self.prev.clone()
//     }
//
//     fn peek(&mut self) -> Option<&Token> {
//         self.tokens.peek()
//     }
//
//     fn next_token_is_either(&mut self, tokens_to_match: &[TokenKind]) -> bool {
//         if let Some(t) = self.tokens.next_if(|t| tokens_to_match.contains(&t.kind)) {
//             self.prev = Some(t);
//             true
//         } else {
//             false
//         }
//     }
//
//     fn parse_expression(&mut self) -> miette::Result<String> {
//         let next_token = match self.peek() {
//             Some(token) => token,
//             None => return Err(MietteError::OutOfBounds.into()),
//         };
//         match next_token.kind {
//             TokenKind::LeftParen => {}
//         }
//         let comparison = self.parse_comparison();
//     }
//
//     fn parse_comparison(&mut self) -> Expression {
//         let term = self.parse_term();
//     }
//
//     fn parse_term(&mut self) -> Expression {
//         let factor = self.parse_factor();
//     }
//
//     fn parse_factor(&mut self) -> Expression {
//         let unary = self.parse_unary();
//     }
//
//     fn parse_unary(&mut self) -> Expression {}
//
//     fn parse_primary(&mut self) -> Expression {}
// }
