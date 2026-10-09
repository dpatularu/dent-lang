#[derive(Debug, Copy, Clone, PartialEq)]
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

    String,
    Number,

    Identifier,

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

    LineComment,
    BlockComment,
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    lexeme: String,
    literal: Option<String>,
    line: usize,
    offset: usize,
}

impl Token {
    pub fn new(
        kind: TokenKind,
        lexeme: &str,
        literal: Option<&str>,
        line: usize,
        offset: usize,
    ) -> Self {
        Self {
            kind,
            lexeme: lexeme.into(),
            literal: literal.map(|x| x.into()),
            line,
            offset,
        }
    }
}
