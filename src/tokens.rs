use std::ops::Range;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    LeftParen,
    RightParen,
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
    QuestionMark,
    Colon,

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
    pub span: Range<usize>,
}

impl Token {
    pub fn new(kind: TokenKind, offset: usize) -> Self {
        let position_end = match &kind {
            TokenKind::LeftParen => 1,
            TokenKind::RightParen => 1,
            TokenKind::LeftBrace => 1,
            TokenKind::RightBrace => 1,
            TokenKind::Comma => 1,
            TokenKind::Dot => 1,
            TokenKind::Minus => 1,
            TokenKind::Plus => 1,
            TokenKind::Star => 1,
            TokenKind::SemiColon => 1,
            TokenKind::Bang => 1,
            TokenKind::Equal => 1,
            TokenKind::Less => 1,
            TokenKind::Greater => 1,
            TokenKind::Slash => 1,
            TokenKind::QuestionMark => 1,
            TokenKind::Colon => 1,
            TokenKind::String(s) => s.len(),
            TokenKind::Number(n) => n.to_string().len(),
            TokenKind::And => 3,
            TokenKind::Class => 5,
            TokenKind::Else => 4,
            TokenKind::False => 5,
            TokenKind::For => 3,
            TokenKind::Fun => 3,
            TokenKind::If => 2,
            TokenKind::Nil => 3,
            TokenKind::Or => 2,
            TokenKind::Print => 5,
            TokenKind::Return => 6,
            TokenKind::Super => 5,
            TokenKind::This => 4,
            TokenKind::True => 4,
            TokenKind::Var => 3,
            TokenKind::While => 5,
            TokenKind::LineComment(c) => c.len(),
            TokenKind::BlockComment(c) => c.len(),
            TokenKind::BangEqual => 2,
            TokenKind::EqualEqual => 2,
            TokenKind::LessEqual => 2,
            TokenKind::GreaterEqual => 2,
            TokenKind::Identifier(s) => s.len(),
        };
        Self {
            kind,
            span: offset..(offset + position_end),
        }
    }
}

impl PartialEq<TokenKind> for Token {
    fn eq(&self, other: &TokenKind) -> bool {
        self.kind == *other
    }
}
