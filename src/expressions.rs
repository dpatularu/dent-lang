#[derive(Debug, Clone)]
pub enum UnaryOperator {
    Negative,
    Not,
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Equal,
    NotEqual,
    And,
    Or,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
    Boolean(bool),
    Nil,
    Group(Box<Expression>),
    Unary {
        operator: UnaryOperator,
        expression: Box<Expression>,
    },
    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    Ternary {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Box<Expression>
    }
}
