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
    If(Box<(WithCursor<Expression>, WithCursor<Block>, Option<Else>)>),
    Match(Box<(Expression, Vec<(Expression, Expression)>)>),
}

pub type Else = WithCursor<Block>;


#[derive(lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum Statement {
    While(Box<(WithCursor<Expression>, WithCursor<Block>)>),
    For(Box<(Variable, WithCursor<Expression>, WithCursor<Block>)>),
    Return(Box<Option<WithCursor<Expression>>>),
    If(Box<WithCursor<Expression>>),
    Expression(Box<WithCursor<Expression>>),
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
            LexerTokenKind::EqualEqual => Self::Equal,
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
            LexerTokenKind::Equal => Self::Assign,
            _ => return Err(ParserErrorKind::ConvertError(value))
        })
    }
}



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Option<Vec<Variable>>,
    pub strict_type: Option<String>,
    pub body: WithCursor<Block>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub strict_type: Option<String>,
    pub value: Option<WithCursor<Expression>>,
}
