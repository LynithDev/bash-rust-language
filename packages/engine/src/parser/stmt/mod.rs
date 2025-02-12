pub use stmt_break::Break;
pub use stmt_constant::Constant;
pub use stmt_continue::Continue;
pub use stmt_expression::ExpressionStmt;
pub use stmt_for::For;
pub use stmt_function::Function;
pub use stmt_if::IfStmt;
pub use stmt_include::Include;
pub use stmt_match::MatchStmt;
pub use stmt_return::Return;
pub use stmt_variable::{VariableMeta, Variable};
pub use stmt_while::While;

mod stmt_break;
mod stmt_constant;
mod stmt_continue;
mod stmt_expression;
mod stmt_for;
mod stmt_function;
mod stmt_if;
mod stmt_include;
mod stmt_match;
mod stmt_return;
mod stmt_variable;
mod stmt_while;

#[derive(lang_macro::EnumVariants, Debug, Clone, PartialEq, Eq)]
pub enum StatementKind {
    Constant(Constant),
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