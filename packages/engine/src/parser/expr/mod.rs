mod literal;
mod bin_op;
mod expr_arithmetic;
mod expr_assignment;
mod expr_block;
mod expr_function_call;
mod expr_group;
mod expr_if;
mod expr_match;
mod expr_range;
mod expr_shell_command;
mod expr_unary;
mod expr_identifier;

pub use bin_op::and_op::And;
pub use bin_op::or_op::Or;
pub use expr_identifier::Identifier;
pub use literal::{boolean::BooleanLiteral, integer::IntegerLiteral, string::StringLiteral};
pub use literal::Literal;
pub use expr_arithmetic::Arithmetic;
pub use expr_assignment::Assignment;
pub use expr_block::Block;
pub use expr_function_call::FunctionCall;
pub use expr_group::Group;
pub use expr_if::IfExpr;
pub use expr_match::MatchExpr;
pub use expr_range::Range;
pub use expr_shell_command::ShellCommand;
pub use expr_unary::Unary;

use crate::cursor::Cursor;

#[derive(lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    Identifier(Identifier),
    String(StringLiteral),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),

    Or(Or),
    And(And),

    Group(Group),
    Unary(Unary),
    Arithmetic(Arithmetic),
    Assignment(Assignment),
    Range(Range),
    ShellCommand(ShellCommand),
    FunctionCall(FunctionCall),
    If(IfExpr),
    Match(MatchExpr),
    Block(Block),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Expression {
    pub start: Cursor,
    pub kind: Box<ExpressionKind>,
    pub end: Cursor,
}

impl Expression {
    pub fn new(start: Cursor, kind: ExpressionKind, end: Cursor) -> Self {
        Self {
            start,
            kind: Box::from(kind),
            end,
        }
    }
}