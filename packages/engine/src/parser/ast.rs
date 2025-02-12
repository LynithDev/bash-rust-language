use super::{expr::ExpressionKind, stmt::{Statement, StatementKind}, Parser, ParserResult};

pub type ProgramTree = Vec<Statement>;

pub trait Parse<T> {
    fn parse(parser: &mut Parser) -> ParserResult<Option<T>>;
}

#[macro_export]
macro_rules! parseable {
    ($name:ident = |$parser:ident| $body:block) => {
        impl $crate::parser::ast::Parse<$name> for $name {
            fn parse($parser: &mut $crate::parser::Parser) -> $crate::parser::ParserResult<Option<$name>> {
                #[allow(unused_imports)]
                use $crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::{expr::ExpressionKind, stmt::StatementKind, ParserErrorKind}};
                
                $body
            }
        }
    };
}

#[macro_export]
macro_rules! parse {
    ($parser:expr, $expr:ident = $name:ty) => {
        let Some($expr) = <$name>::parse($parser)? else {
            return Ok(None);
        };
    };
}

#[macro_export]
macro_rules! ast {
    ($name:ident { $($param_k:ident: $param_v:ty),* $(,)* }) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name {
            $(pub $param_k: $param_v),*
        }
    };

    ($name:ident($($param:ty),*) $(,)*) => {
        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct $name($(pub $param),*);
    };
}

pub trait ToExpressionKind {
    fn as_expr_kind(self) -> ExpressionKind;
}

#[macro_export]
macro_rules! to_expr_kind {
    ($name:ident) => {
        $crate::to_expr_kind!($name = $name);
    };

    ($name:ident = $kind:ident) => {
        impl $crate::parser::ast::ToExpressionKind for $name {
            fn as_expr_kind(self) -> $crate::parser::expr::ExpressionKind {
                $crate::parser::expr::ExpressionKind::$kind(self)
            }
        }
    };
}

pub trait ToStatementKind {
    fn as_stmt_kind(self) -> StatementKind;
}

#[macro_export]
macro_rules! as_stmt_kind {
    ($name:ident) => {
        $crate::as_stmt_kind!($name = $name);
    };

    ($name:ident = $kind:ident) => {
        impl $crate::parser::ast::ToStatementKind for $name {
            fn as_stmt_kind(self) -> $crate::parser::stmt::StatementKind {
                $crate::parser::stmt::StatementKind::$kind(self)
            }
        }
    };
}