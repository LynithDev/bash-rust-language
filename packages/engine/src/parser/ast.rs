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

pub trait ToExpression {
    fn as_expr(self) -> ExpressionKind;
}

#[macro_export]
macro_rules! as_expr {
    ($name:ident) => {
        $crate::as_expr!($name = $name);
    };

    ($name:ident = $kind:ident) => {
        impl $crate::parser::ast::ToExpression for $name {
            fn as_expr(self) -> $crate::parser::expr::ExpressionKind {
                $crate::parser::expr::ExpressionKind::$kind(self)
            }
        }
    };
}

pub trait ToStatement {
    fn as_stmt(self) -> StatementKind;
}

#[macro_export]
macro_rules! as_stmt {
    ($name:ident) => {
        $crate::as_stmt!($name = $name);
    };

    ($name:ident = $kind:ident) => {
        impl $crate::parser::ast::ToStatement for $name {
            fn as_stmt(self) -> $crate::parser::stmt::StatementKind {
                $crate::parser::stmt::StatementKind::$kind(self)
            }
        }
    };
}