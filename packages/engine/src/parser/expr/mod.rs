mod literal;
mod bin_op;
mod arithmetic_expr;
mod assignment_expr;
mod block_expr;
mod function_call_expr;
mod group_expr;
mod if_expr;
mod match_expr;
mod range_expr;
mod shell_command_expr;
mod unary_expr;
mod identifier;

pub use identifier::Identifier;
use literal::{boolean::BooleanLiteral, integer::IntegerLiteral, string::StringLiteral};
pub use literal::Literal;
pub use arithmetic_expr::Arithmetic;
pub use assignment_expr::Assignment;
pub use block_expr::Block;
pub use function_call_expr::FunctionCall;
pub use group_expr::Group;
pub use if_expr::IfExpr;
pub use match_expr::MatchExpr;
pub use range_expr::Range;
pub use shell_command_expr::ShellCommand;
pub use unary_expr::Unary;

use crate::cursor::Cursor;

#[derive(lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum ExpressionKind {
    Identifier(Identifier),
    String(StringLiteral),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),
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
    start: Cursor,
    end: Cursor,
    kind: Box<ExpressionKind>,
}