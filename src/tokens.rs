#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LeftParan,
    RightParan,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Star,
    SemiColon,
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    LessEqual,
    GreaterEqual,
    Less,
    Greater,
    Slash,
    Eof,

    String(String),
    Number(f64),

    Identifier(String),

    And,
    Class,
    Else,
    False,
    For,
    Fun,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    LineComment(String),
    BlockComment(String),
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    line: usize,
    offset: usize,
}

impl Token {
    pub fn new(kind: TokenKind, line: usize, offset: usize) -> Self {
        Self { kind, line, offset }
    }
}
