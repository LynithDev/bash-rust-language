use std::{iter::Peekable, slice::Iter};

use ast::{
    ArithmeticOperator, AssignmentOperator, Block, Expression, Function, Identifier, Literal,
    LogicalOperator, ProgramTree, Statement, UnaryOperator, Variable,
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
    token: Option<LexerToken>,
    cursor: Cursor,
    errors: ErrorList,

    #[cfg(feature = "cli")]
    source: crate::error::SourceFile,
}

macro_rules! let_expr {
    ($name:ident = $exp:expr) => {
        let Some($name) = $exp else {
            return Ok(None);
        };
    };

    (mut $name:ident = $exp:expr) => {
        let Some(mut $name) = $exp else {
            return Ok(None);
        };
    };
}

impl<'a> Parser<'a> {
    pub fn create(
        tokens: &'a LexerTokenList,
        #[cfg(feature = "cli")] source: crate::error::SourceFile,
    ) -> Self {
        let parser = Self {
            tokens: tokens.iter().peekable(),
            token: None,
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

        while let Some(token) = self.peek().cloned() {
            match self.parse_statement(token) {
                Ok(Some(statement)) => statements.push(statement),
                Err(err) => {
                    let token = self.token.as_ref().unwrap_or(token);
                    self.add_error(token.start, token.end, err)
                }
                _ => warn!("empty")
            }

            self.next();
        }

        statements
    }

    fn add_error(&mut self, start: Cursor, end: Cursor, kind: ParserErrorKind) {
        self.errors.push(EngineErrorKind::ParserError(ParserError {
            start,
            end,
            kind: Box::new(kind),

            #[cfg(feature = "cli")]
            source_file: self.get_source_sliced(start, end),
        }));
    }

    fn parse_statement(&mut self, token: &LexerToken) -> ParserResult<Option<Statement>> {
        use LexerTokenKind::*;
        Ok(Some(match &token.kind {
            Var => self.var_decl()?,
            Function => self.func_decl()?,
            If => self.parse_if()?,
            EOF | EOL => return Ok(None),

            _ => {
                let_expr!(expr = self.expression()?);
                Statement::Expression(Box::new(expr))
            }
        }))
    }

    fn var_decl(&mut self) -> ParserResult<Statement> {
        self.expect_token(&LexerTokenKind::Var)?;
        let value = self.parse_var()?;
        self.expect_terminator()?;

        Ok(Statement::Variable(Box::from(value)))
    }

    fn parse_var(&mut self) -> ParserResult<Variable> {
        let identifier = self
            .expect_token(&LexerTokenKind::Identifier)?
            .as_identifier()?
            .clone();

        let strict_type = self.parse_explicit_type()?;

        let value = if self.next_if_eq(&&LexerTokenKind::Equal).is_some() {
            self.expression()?
        } else {
            None
        };

        let variable = Variable {
            name: identifier.to_owned(),
            strict_type,
            value,
        };

        Ok(variable)
    }

    fn parse_explicit_type(&mut self) -> ParserResult<Option<String>> {
        Ok(if self.next_if_eq(&&LexerTokenKind::Colon).is_some() {
            let identifier = self
                .expect_token(&LexerTokenKind::Identifier)?
                .as_identifier()?
                .clone();

            Some(identifier)
        } else {
            None
        })
    }

    fn func_decl(&mut self) -> ParserResult<Statement> {
        self.expect_token(&LexerTokenKind::Function)?;

        let identifier = self
            .expect_token(&LexerTokenKind::Identifier)?
            .as_identifier()?
            .clone();

        self.expect_token(&LexerTokenKind::LParen)?;

        let parameters = if self.next_if_eq(&&LexerTokenKind::RParen).is_none() {
            let mut variables = vec![];

            while let Some(token) = self.peek() {
                if token.kind == LexerTokenKind::RParen {
                    break;
                }

                if self.next_if_eq(&&LexerTokenKind::Comma).is_some() {
                    continue;
                }

                variables.push(self.parse_var()?);
            }

            self.next();

            Some(variables)
        } else {
            None
        };

        let strict_type = self.parse_explicit_type()?;

        self.expect_token(&LexerTokenKind::LBracket)?;

        let body = self.parse_block()?;

        let function = Function {
            name: identifier.to_owned(),
            parameters,
            strict_type,
            body,
        };

        Ok(Statement::Function(Box::from(function)))
    }

    fn parse_if(&mut self) -> ParserResult<Statement> {
        self.expect_token(&LexerTokenKind::If)?;

        if let Some(expr) = self.if_expr()? {
            Ok(Statement::If(Box::from(expr)))
        } else {
            Err(ParserErrorKind::UnexpectedEnd)
        }
    }

    fn if_expr(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(condition = self.expression()?);

        let start = self.cursor;
        
        
        let truthy_block = if self.next_if_eq(&&LexerTokenKind::LBracket).is_some() {
            self.parse_block()?
        } else if self.next_if_eq(&&LexerTokenKind::Colon).is_some() {
            self.parse_inline_block()?
        } else {
            return Err(ParserErrorKind::ExpectedExpression);
        };

        self.next();

        let else_condition = if self.next_if_eq(&&LexerTokenKind::Else).is_some() {
            let start = self.cursor;

            Some(match self.peek().map(|t| t.kind.clone()) {
                Some(LexerTokenKind::LBracket) => {
                    self.next();
                    self.parse_block()?
                },
                Some(LexerTokenKind::Colon) => {
                    self.next();
                    self.parse_inline_block()?
                },
                _ => {
                    let stmt = self.parse_if()?;
                    WithCursor::create_with(start, self.cursor, vec![stmt])
                },
            })
        } else {
            None
        };
        
        let if_expr = Expression::If(Box::from((condition, truthy_block, else_condition)));
        
        Ok(Some(WithCursor::create_with(start, self.cursor, if_expr)))
    }

    fn parse_inline_block(&mut self) -> ParserResult<WithCursor<Block>> {
        let Some(next) = self.next() else {
            return Err(ParserErrorKind::ExpectedStatement);
        };

        let start = self.cursor;
        let Some(stmt) = self.parse_statement(next)? else {
            return Err(ParserErrorKind::ExpectedStatement);
        };

        Ok(WithCursor::create_with(start, self.cursor, vec![stmt]))
    }


    fn parse_block(&mut self) -> ParserResult<WithCursor<Block>> {
        let mut block = Block::new();
        let start = self.cursor;

        while let Some(token) = self.peek().cloned() {
            if token.kind == LexerTokenKind::RBracket {
                break;
            }

            if let Some(statement) = self.parse_statement(token)? {
                block.push(statement);
            }

            self.next();
        }

        Ok(WithCursor::create_with(start, self.cursor, block))
    }

    fn expression(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        self.assignment()
    }

    fn assignment(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(lhs = self.logic_or()?);

        if let Some(op_token) = self.next_if_eq_mul(&[
            &LexerTokenKind::Equal,
            &LexerTokenKind::PlusEqual,
            &LexerTokenKind::MinusEqual,
            &LexerTokenKind::MultiplyEqual,
            &LexerTokenKind::DivideEqual,
        ]) {
            let_expr!(rhs = self.logic_or()?);

            if let WithCursor {
                value: Expression::Identifier(_),
                ..
            } = lhs
            {
                let operator: AssignmentOperator = op_token.kind.clone().try_into()?;

                return Ok(Some(WithCursor::create_with(
                    lhs.start,
                    rhs.end,
                    Expression::Assignment(Box::from((
                        lhs,
                        WithCursor::create_with(op_token.start, op_token.end, operator),
                        rhs,
                    ))),
                )));
            }
        }

        Ok(Some(lhs))
    }

    fn logic_or(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.logic_and()?);

        while let Some(token_or) = self.next_if_eq(&&LexerTokenKind::Or) {
            let_expr!(rhs = self.logic_and()?);

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

        Ok(Some(lhs))
    }

    fn logic_and(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.cmp_equality()?);

        while let Some(token_and) = self.next_if_eq(&&LexerTokenKind::And) {
            let_expr!(rhs = self.cmp_equality()?);

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

        Ok(Some(lhs))
    }

    fn cmp_equality(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.cmp_lgt()?);

        while let Some(token_eq) =
            self.next_if_eq_mul(&[&LexerTokenKind::EqualEqual, &LexerTokenKind::NotEqual])
        {
            let_expr!(rhs = self.cmp_lgt()?);

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

        Ok(Some(lhs))
    }

    fn cmp_lgt(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.arith_add_sub()?);

        while let Some(token_cmp) = self.next_if_eq_mul(&[
            &LexerTokenKind::LesserThan,
            &LexerTokenKind::LesserEqualThan,
            &LexerTokenKind::GreaterThan,
            &LexerTokenKind::GreaterEqualThan,
        ]) {
            let_expr!(rhs = self.arith_add_sub()?);

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

        Ok(Some(lhs))
    }

    fn arith_add_sub(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.arith_mul_div()?);

        while let Some(token_arith) =
            self.next_if_eq_mul(&[&LexerTokenKind::Plus, &LexerTokenKind::Minus])
        {
            let_expr!(rhs = self.arith_mul_div()?);

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

        Ok(Some(lhs))
    }

    fn arith_mul_div(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.unary()?);

        while let Some(token_arith) =
            self.next_if_eq_mul(&[&LexerTokenKind::Multiply, &LexerTokenKind::Divide])
        {
            let_expr!(rhs = self.unary()?);

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

        Ok(Some(lhs))
    }

    fn unary(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.func_invoke()?);

        while let Some(token_unary) = self.next_if_eq(&&LexerTokenKind::Not) {
            let_expr!(rhs = self.unary()?);

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

        Ok(Some(lhs))
    }

    fn func_invoke(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        self.literals() // TODO
    }

    fn literals(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(token = self.peek());

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

            LexerTokenKind::Identifier => {
                Expression::Identifier(Box::from(Identifier::from(token.as_identifier()?)))
            }

            LexerTokenKind::ShellCommand => {
                Expression::ShellCommand(Box::from(token.as_shell_command()?.to_owned()))
            }

            LexerTokenKind::EOL | LexerTokenKind::EOF => return Ok(None),

            _ => {
                let token = token.to_owned();
                self.cursor_next(&token);
                return Err(ParserErrorKind::UnexpectedToken(token.kind.clone()))
            },
        };

        let token = token.to_owned();
        self.next();

        Ok(Some(WithCursor::create_with(token.start, token.end, expr)))
    }

    fn expect_token(&mut self, expected: &'a LexerTokenKind) -> ParserResult<&LexerToken> {
        self.expect(&expected).map_err(|found| {
            ParserErrorKind::ExpectedToken(vec![expected.clone()], found.map(|t| t.kind.clone()))
        })
    }

    fn expect_terminator(&mut self) -> ParserResult<()> {
        match self.peek().map(|t| t.kind.clone()) {
            Some(LexerTokenKind::EOL) | Some(LexerTokenKind::EOF) | None => {
                Ok(())
            },
            found => {
                Err(ParserErrorKind::ExpectedToken(vec![LexerTokenKind::EOL, LexerTokenKind::EOF], found))
            }
        }
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
        self.token = Some(item.to_owned().to_owned());

        self.cursor.clone_from(&item.end);
    }
}
