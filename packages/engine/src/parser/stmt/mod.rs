pub use break_stmt::Break;
pub use continue_stmt::Continue;
pub use expression_stmt::ExpressionStmt;
pub use for_stmt::For;
pub use function_stmt::Function;
pub use if_stmt::IfStmt;
pub use include_stmt::Include;
pub use match_stmt::MatchStmt;
pub use return_stmt::Return;
pub use variable_stmt::{VariableMeta, Variable};
pub use while_stmt::While;

mod break_stmt;
mod constant_stmt;
mod continue_stmt;
mod expression_stmt;
mod for_stmt;
mod function_stmt;
mod if_stmt;
mod include_stmt;
mod match_stmt;
mod return_stmt;
mod variable_stmt;
mod while_stmt;

#[derive(lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum StatementKind {
    While(While),
    For(For),
    Return(Return),
    If(IfStmt),
    Match(MatchStmt),
    Expression(ExpressionStmt),
    Continue(Continue),
    Break(Break),
    Variable(Variable),
    Function(Function),
    Include(Include),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Statement {
    kind: Box<StatementKind>,
}