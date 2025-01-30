use crate::cursor::WithCursor;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(WithCursor<Literal>),
    Group(Box<WithCursor<Expression>>),
    Unary(Box<WithCursor<UnaryOperator>>),
    Binary(Box<(WithCursor<Expression>, WithCursor<BinaryOperator>, WithCursor<Expression>)>),
    Logical(Box<(WithCursor<Expression>, WithCursor<LogicalOperator>, WithCursor<Expression>)>),
    Range(Box<(WithCursor<Literal>, WithCursor<Literal>, bool)>),
    ShellCommand(Box<(String, String)>),
    VariableUse(Box<Identifier>),
    FunctionCall(Box<(Identifier, Vec<Expression>)>),
    If(Box<(Expression, Block, Option<Statement>)>),
    Match(Box<(Expression, Vec<(Expression, Expression)>)>)
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    While(Box<(Expression, Block)>),
    For(Box<(Variable, Expression, Block)>),
    Return(Box<Option<Expression>>),
    Continue,
    Break,
    Variable(Box<Variable>),
    Constant(Box<Variable>),
    Function(Box<Function>),
    Include(Box<String>),
}

pub type ProgramTree = Block;
pub type Identifier = String;
pub type Block = Vec<Statement>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(isize),
    Boolean(bool),
    String(Box<String>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Negative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogicalOperator {
    Equal,
    NotEqual,
    LesserThan,
    LesserEqualThan,
    GreaterThan,
    GreaterEqualThan,
    And,
    Or
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    name: String,
    parameters: Vec<Variable>,
    body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    name: String,
}
