use std::ops::Range;

use crate::syntax::expressions::Expression;

#[derive(Debug)]
pub enum StatementKind {
    ExpressionStatement(Expression),
    PrintStatement(Expression),
    VariableDeclaration(String, Expression),
}

#[derive(Debug)]

pub struct Statement {
    pub kind: StatementKind,
    pub span: Range<usize>
}

impl Statement {
    pub fn new(kind: StatementKind, span: Range<usize>) -> Self {
        Self { kind, span }
    }
}