pub enum Expression {
    Number(f64),
    String(String),
    True,
    False,
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
}

pub enum UnaryOperator {
    Negative,
    Not,
}

pub enum BinaryOperator {
    BoolEqual,
    BoolNotEqual,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Plus,
    Minus,
    Multiply,
    Divide,
}
