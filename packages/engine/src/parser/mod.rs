use std::{iter::Peekable, slice::Iter};

use ast::{
    ArithmeticOperator, AssignmentOperator, Expression, Literal, LogicalOperator, ProgramTree,
    Statement, UnaryOperator, Variable,
};
use error::ParserResult;

use crate::{
    component::{ComponentErrors, ComponentIter},
    cursor::WithCursor,
    error::{EngineErrorKind, ErrorList},
    lexer::tokens::{LexerToken, LexerTokenKind, LexerTokenList},
    Cursor,
};

pub use error::{ParserError, ParserErrorKind};

pub mod ast;
mod error;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, LexerToken>>,
    cursor: Cursor,
    errors: ErrorList,

    #[cfg(feature = "cli")]
    source: crate::error::SourceFile,
}

impl<'a> Parser<'a> {
    pub fn create(
        tokens: &'a LexerTokenList,
        #[cfg(feature = "cli")] source: crate::error::SourceFile,
    ) -> Self {
        let parser = Self {
            tokens: tokens.iter().peekable(),
            cursor: Cursor::create(),
            errors: ErrorList::new(),

            #[cfg(feature = "cli")]
            source,
        };

        debug!("created parser");
        parser
    }

    pub fn parse(&mut self) -> ProgramTree {
        let mut statements = ProgramTree::new();

        while let Some(token) = self.next() {
            match self.parse_statement(token) {
                Ok(Some(statement)) => statements.push(statement),
                Err(err) => {
                    self.errors.push(EngineErrorKind::ParserError(ParserError {
                        start: token.start,
                        end: self.cursor,
                        kind: Box::new(err),

                        #[cfg(feature = "cli")]
                        source_file: self.get_source_sliced(token.start, self.cursor),
                    }));
                },
                _ => {}
            }
        }

        statements
    }

    fn parse_statement(&mut self, token: &LexerToken) -> ParserResult<Option<Statement>> {
        use LexerTokenKind::*;
        Ok(Some(match &token.kind {
            Var => self.var_decl()?,
            EOF => return Ok(None),

            _ => Statement::Expression(Box::new(self.expression()?)),
        }))
    }

    fn var_decl(&mut self) -> ParserResult<Statement> {
        let identifier = self
            .expect_token(&LexerTokenKind::Identifier)?
            .as_identifier()?
            .clone();

        let value = if self.next_if_eq(&&LexerTokenKind::Equal).is_some() {
            Some(Box::from(self.expression()?))
        } else {
            None
        };

        let variable = Variable {
            name: Box::from(identifier.to_owned()),
            value,
        };

        Ok(Statement::Variable(Box::from(variable)))
    }

    fn expression(&mut self) -> ParserResult<WithCursor<Expression>> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<WithCursor<Expression>> {
        let lhs = self.logic_or()?;

        if let Some(op_token) = self.next_if_eq_mul(&[
            &LexerTokenKind::EqualEqual,
            &LexerTokenKind::PlusEqual,
            &LexerTokenKind::MinusEqual,
            &LexerTokenKind::MultiplyEqual,
            &LexerTokenKind::DivideEqual,
        ]) {
            let rhs = self.logic_or()?;

            if let WithCursor {
                value: Expression::Identifier(_),
                ..
            } = lhs
            {
                let operator: AssignmentOperator = op_token.kind.clone().try_into()?;

                return Ok(WithCursor::create_with(
                    lhs.start,
                    rhs.end,
                    Expression::Assignment(Box::from((
                        lhs,
                        WithCursor::create_with(op_token.start, op_token.end, operator),
                        rhs,
                    ))),
                ));
            }
        }

        Ok(lhs)
    }

    fn logic_or(&mut self) -> ParserResult<WithCursor<Expression>> {
        let mut lhs = self.logic_and()?;

        while let Some(token_or) = self.next_if_eq(&&LexerTokenKind::Or) {
            let rhs = self.logic_and()?;

            let operator: LogicalOperator = LogicalOperator::Or;

            lhs = WithCursor::create_with(
                lhs.start,
                rhs.end,
                Expression::Logical(Box::from((
                    lhs,
                    WithCursor::create_with(token_or.start, token_or.end, operator),
                    rhs,
                ))),
            );
        }

        Ok(lhs)
    }

    fn logic_and(&mut self) -> ParserResult<WithCursor<Expression>> {
        let mut lhs = self.cmp_equality()?;

        while let Some(token_and) = self.next_if_eq(&&LexerTokenKind::And) {
            let rhs = self.cmp_equality()?;

            let operator: LogicalOperator = LogicalOperator::And;

            lhs = WithCursor::create_with(
                lhs.start,
                rhs.end,
                Expression::Logical(Box::from((
                    lhs,
                    WithCursor::create_with(token_and.start, token_and.end, operator),
                    rhs,
                ))),
            );
        }

        Ok(lhs)
    }

    fn cmp_equality(&mut self) -> ParserResult<WithCursor<Expression>> {
        let mut lhs = self.cmp_lgt()?;

        while let Some(token_eq) =
            self.next_if_eq_mul(&[&LexerTokenKind::EqualEqual, &LexerTokenKind::NotEqual])
        {
            let rhs = self.cmp_lgt()?;

            let operator: LogicalOperator = token_eq.kind.clone().try_into()?;

            lhs = WithCursor::create_with(
                lhs.start,
                rhs.end,
                Expression::Logical(Box::from((
                    lhs,
                    WithCursor::create_with(token_eq.start, token_eq.end, operator),
                    rhs,
                ))),
            );
        }

        Ok(lhs)
    }

    fn cmp_lgt(&mut self) -> ParserResult<WithCursor<Expression>> {
        let mut lhs = self.arith_add_sub()?;

        while let Some(token_cmp) = self.next_if_eq_mul(&[
            &LexerTokenKind::LesserThan,
            &LexerTokenKind::LesserEqualThan,
            &LexerTokenKind::GreaterThan,
            &LexerTokenKind::GreaterEqualThan,
        ]) {
            let rhs = self.arith_add_sub()?;

            let operator: LogicalOperator = token_cmp.kind.clone().try_into()?;

            lhs = WithCursor::create_with(
                lhs.start,
                rhs.end,
                Expression::Logical(Box::from((
                    lhs,
                    WithCursor::create_with(token_cmp.start, token_cmp.end, operator),
                    rhs,
                ))),
            );
        }

        Ok(lhs)
    }

    fn arith_add_sub(&mut self) -> ParserResult<WithCursor<Expression>> {
        let mut lhs = self.arith_mul_div()?;

        while let Some(token_arith) =
            self.next_if_eq_mul(&[&LexerTokenKind::Plus, &LexerTokenKind::Minus])
        {
            let rhs = self.arith_mul_div()?;

            let operator: ArithmeticOperator = token_arith.kind.clone().try_into()?;

            lhs = WithCursor::create_with(
                lhs.start,
                rhs.end,
                Expression::Arithmetic(Box::from((
                    lhs,
                    WithCursor::create_with(token_arith.start, token_arith.end, operator),
                    rhs,
                ))),
            );
        }

        Ok(lhs)
    }

    fn arith_mul_div(&mut self) -> ParserResult<WithCursor<Expression>> {
        let mut lhs = self.unary()?;

        while let Some(token_arith) =
            self.next_if_eq_mul(&[&LexerTokenKind::Multiply, &LexerTokenKind::Divide])
        {
            let rhs = self.unary()?;

            let operator: ArithmeticOperator = token_arith.kind.clone().try_into()?;

            lhs = WithCursor::create_with(
                lhs.start,
                rhs.end,
                Expression::Arithmetic(Box::from((
                    lhs,
                    WithCursor::create_with(token_arith.start, token_arith.end, operator),
                    rhs,
                ))),
            );
        }

        Ok(lhs)
    }

    fn unary(&mut self) -> ParserResult<WithCursor<Expression>> {
        let mut lhs = self.func_invoke()?;

        while let Some(token_unary) = self.next_if_eq(&&LexerTokenKind::Not) {
            let rhs = self.unary()?;

            let operator: UnaryOperator = token_unary.kind.clone().try_into()?;

            lhs = WithCursor::create_with(
                lhs.start,
                rhs.end,
                Expression::Unary(Box::from((
                    WithCursor::create_with(token_unary.start, token_unary.end, operator),
                    lhs,
                ))),
            );
        }

        Ok(lhs)
    }

    fn func_invoke(&mut self) -> ParserResult<WithCursor<Expression>> {
        self.literals() // TODO
    }

    fn literals(&mut self) -> ParserResult<WithCursor<Expression>> {
        let Some(token) = self.next() else {
            return Err(ParserErrorKind::UnexpectedEnd);
        };

        let expr = match token.kind {
            LexerTokenKind::String => Expression::Literal(Box::from(Literal::String(Box::from(
                token.as_string()?.to_owned(),
            )))),
            LexerTokenKind::Integer => {
                Expression::Literal(Box::from(Literal::Integer(*token.as_integer()?)))
            }
            LexerTokenKind::Boolean => {
                Expression::Literal(Box::from(Literal::Boolean(*token.as_boolean()?)))
            }
            LexerTokenKind::ShellCommand => {
                Expression::ShellCommand(Box::from(token.as_shell_command()?.to_owned()))
            }

            _ => return Err(ParserErrorKind::UnexpectedToken(token.kind.clone())),
        };

        Ok(WithCursor::create_with(token.start, token.end, expr))
    }

    fn expect_token(&mut self, expected: &'a LexerTokenKind) -> ParserResult<&LexerToken> {
        self.expect(&expected).map_err(|found| {
            ParserErrorKind::ExpectedToken(expected.clone(), found.map(|t| t.kind.clone()))
        })
    }
}

impl ComponentErrors for Parser<'_> {
    fn fetch_errors(&self) -> &ErrorList {
        &self.errors
    }

    #[cfg(feature = "cli")]
    fn source(&self) -> &crate::error::SourceFile {
        &self.source
    }
}

impl<'a> ComponentIter<'a, &'a LexerTokenKind, &'a LexerToken, Iter<'a, LexerToken>>
    for Parser<'a>
{
    fn get_iter(&mut self) -> &mut Peekable<Iter<'a, LexerToken>> {
        &mut self.tokens
    }

    fn cursor_next(&mut self, item: &&LexerToken) {
        self.cursor.clone_from(&item.end);
    }
}
