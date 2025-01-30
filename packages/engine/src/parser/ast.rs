use std::fmt::Debug;

use crate::{cursor::WithCursor, lexer::tokens::{LexerToken, LexerTokenKind, ShellCommand}};

use super::ParserErrorKind;

#[derive(lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum Expression {
    Literal(Box<Literal>),
    Group(Box<Expression>),
    Unary(Box<(WithCursor<UnaryOperator>, WithCursor<Expression>)>),
    Arithmetic(Box<(WithCursor<Expression>, WithCursor<ArithmeticOperator>, WithCursor<Expression>)>),
    Logical(Box<(WithCursor<Expression>, WithCursor<LogicalOperator>, WithCursor<Expression>)>),
    Assignment(Box<(WithCursor<Expression>, WithCursor<AssignmentOperator>, WithCursor<Expression>)>),
    Range(Box<(WithCursor<Literal>, WithCursor<Literal>, bool)>),
    ShellCommand(Box<ShellCommand>),
    Identifier(Box<Identifier>),
    FunctionCall(Box<(Identifier, Vec<Expression>)>),
    If(Box<(Expression, Block, Option<Statement>)>),
    Match(Box<(Expression, Vec<(Expression, Expression)>)>),
    Empty,
}



#[derive(lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    While(Box<(Expression, Block)>),
    For(Box<(Variable, Expression, Block)>),
    Return(Box<Option<Expression>>),
    Expression(Box<Expression>),
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

#[derive(lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum Literal {
    Integer(isize),
    Boolean(bool),
    String(Box<String>),
}

impl TryFrom<LexerToken> for Literal {
    type Error = crate::parser::ParserErrorKind;
    
    fn try_from(value: LexerToken) -> Result<Self, Self::Error> {
        Ok(match value.kind {
            LexerTokenKind::Integer => Self::Integer(*value.as_integer()?),
            LexerTokenKind::Boolean => Self::Boolean(*value.as_boolean()?),
            LexerTokenKind::String => Self::String(Box::from(value.as_string()?.to_owned())),
            _ => return Err(ParserErrorKind::ConvertError(value.kind))
        })
    }
}



#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArithmeticOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl TryFrom<LexerTokenKind> for ArithmeticOperator {
    type Error = crate::parser::ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::Plus => Self::Add,
            LexerTokenKind::Minus => Self::Subtract,
            LexerTokenKind::Multiply => Self::Multiply,
            LexerTokenKind::Divide => Self::Divide,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}



#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOperator {
    Not,
    Negative,
}

impl TryFrom<LexerTokenKind> for UnaryOperator {
    type Error = crate::parser::ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::Not => Self::Not,
            LexerTokenKind::Minus => Self::Negative,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}



#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
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

impl TryFrom<LexerTokenKind> for LogicalOperator {
    type Error = crate::parser::ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::Equal => Self::Equal,
            LexerTokenKind::NotEqual => Self::NotEqual,
            LexerTokenKind::LesserThan => Self::LesserThan,
            LexerTokenKind::LesserEqualThan => Self::LesserEqualThan,
            LexerTokenKind::GreaterThan => Self::GreaterThan,
            LexerTokenKind::GreaterEqualThan => Self::GreaterEqualThan,
            LexerTokenKind::And => Self::And,
            LexerTokenKind::Or => Self::Or,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}


#[derive(lang_macro::EnumVariants, Debug, Clone, Copy, PartialEq, Eq)]
pub enum AssignmentOperator {
    PlusAssign,
    MinusAssign,
    MultiplyAssign,
    DivideAssign,
    Assign
}

impl TryFrom<LexerTokenKind> for AssignmentOperator {
    type Error = crate::parser::ParserErrorKind;
    
    fn try_from(value: LexerTokenKind) -> Result<Self, Self::Error> {
        Ok(match value {
            LexerTokenKind::PlusEqual => Self::PlusAssign,
            LexerTokenKind::MinusEqual => Self::MinusAssign,
            LexerTokenKind::MultiplyEqual => Self::MultiplyAssign,
            LexerTokenKind::DivideEqual => Self::DivideAssign,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Variable>,
    pub body: Block,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: Box<String>,
    pub value: Option<Box<WithCursor<Expression>>>,
}
