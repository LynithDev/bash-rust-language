use std::{iter::Peekable, rc::Rc, slice::Iter};

use ast::{
    ArithmeticOperator, AssignmentOperator, Block, Expression, Function, Identifier, Literal,
    LogicalOperator, MatchCase, ProgramTree, Statement, UnaryOperator, Variable,
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

// MARK: Parser Struct
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

    // MARK: Parser Main
    pub fn parse(&mut self) -> ProgramTree {
        let mut statements = ProgramTree::new();

        while let Some(token) = self.peek().cloned() {
            match self.parse_statement(token) {
                Ok(Some(statement)) => statements.push(statement),
                Err(err) => {
                    debug!("adding error {err:?}");
                    let token = self.token.as_ref().unwrap_or(token);
                    self.add_error(token.start, token.end, err)
                }
                _ => debug!("empty statement"),
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

    // MARK: Statement
    fn parse_statement(&mut self, token: &LexerToken) -> ParserResult<Option<Statement>> {
        use LexerTokenKind::*;
        Ok(match &token.kind {
            Var => self.stmt_var()?,
            Function => self.stmt_func()?,
            If => self.stmt_if()?,
            Match => self.stmt_match()?,
            For => self.stmt_for()?,
            While => self.stmt_while()?,
            EOF | EOL => return Ok(None),

            _ => {
                let_expr!(expr = self.expression()?);
                Some(Statement::Expression(Box::new(expr)))
            }
        })
    }

    // MARK: Variable
    fn stmt_var(&mut self) -> ParserResult<Option<Statement>> {
        self.expect_token(&LexerTokenKind::Var)?;
        let value = self.parse_var()?;
        self.expect_terminator()?;

        Ok(Some(Statement::Variable(Box::from(value))))
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

    // MARK: Function
    fn stmt_func(&mut self) -> ParserResult<Option<Statement>> {
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

        let body = self.stmt_block()?;

        let function = Function {
            name: identifier.to_owned(),
            parameters,
            strict_type,
            body,
        };

        Ok(Some(Statement::Function(Box::from(function))))
    }

    // MARK: For
    fn stmt_for(&mut self) -> ParserResult<Option<Statement>> {
        self.expect_token(&LexerTokenKind::For)?;

        let identifier = self
            .expect_token(&LexerTokenKind::Identifier)?
            .as_identifier()?;
        let variable = Variable {
            name: identifier.clone(),
            strict_type: None,
            value: None,
        };

        self.expect_token(&LexerTokenKind::In)?;
        let_expr!(expr = self.expression()?);
        self.next();

        let block = self.stmt_block()?;

        Ok(Some(Statement::For(Box::from((variable, expr, block)))))
    }

    // MARK: While
    fn stmt_while(&mut self) -> ParserResult<Option<Statement>> {
        self.expect_token(&LexerTokenKind::While)?;

        let_expr!(condition = self.expression()?);
        self.next();

        let block = self.stmt_block()?;

        Ok(Some(Statement::While(Box::from((condition, block)))))
    }

    fn expression(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        self.expr_assignment()
    }

    // MARK: Assignment
    fn expr_assignment(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(lhs = self.expr_logic_or()?);

        if let Some(op_token) = self.next_if_eq_mul(&[
            &LexerTokenKind::Equal,
            &LexerTokenKind::PlusEqual,
            &LexerTokenKind::MinusEqual,
            &LexerTokenKind::MultiplyEqual,
            &LexerTokenKind::DivideEqual,
        ]) {
            let_expr!(rhs = self.expr_logic_or()?);

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

    // MARK: Logic OR
    fn expr_logic_or(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.expr_logic_and()?);

        while let Some(token_or) = self.next_if_eq(&&LexerTokenKind::Or) {
            let_expr!(rhs = self.expr_logic_and()?);

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

    // MARK: Logic AND
    fn expr_logic_and(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.expr_cmp_equality()?);

        while let Some(token_and) = self.next_if_eq(&&LexerTokenKind::And) {
            let_expr!(rhs = self.expr_cmp_equality()?);

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

    // MARK: Cmp Equal
    fn expr_cmp_equality(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.expr_cmp_lgt()?);

        while let Some(token_eq) =
            self.next_if_eq_mul(&[&LexerTokenKind::EqualEqual, &LexerTokenKind::NotEqual])
        {
            let_expr!(rhs = self.expr_cmp_lgt()?);

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

    // MARK: Cmp LGT
    fn expr_cmp_lgt(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.expr_arith_add_sub()?);

        while let Some(token_cmp) = self.next_if_eq_mul(&[
            &LexerTokenKind::LesserThan,
            &LexerTokenKind::LesserEqualThan,
            &LexerTokenKind::GreaterThan,
            &LexerTokenKind::GreaterEqualThan,
        ]) {
            let_expr!(rhs = self.expr_arith_add_sub()?);

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

    // MARK: Arith Add Sub
    fn expr_arith_add_sub(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.expr_arith_mul_div()?);

        while let Some(token_arith) =
            self.next_if_eq_mul(&[&LexerTokenKind::Plus, &LexerTokenKind::Minus])
        {
            let_expr!(rhs = self.expr_arith_mul_div()?);

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

    // MARK: Arith Mul Div
    fn expr_arith_mul_div(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.expr_unary()?);

        while let Some(token_arith) =
            self.next_if_eq_mul(&[&LexerTokenKind::Multiply, &LexerTokenKind::Divide])
        {
            let_expr!(rhs = self.expr_unary()?);

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

    // MARK: Unary
    fn expr_unary(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut lhs = self.expr_func_invoke()?);

        while let Some(token_unary) = self.next_if_eq(&&LexerTokenKind::Not) {
            let_expr!(rhs = self.expr_unary()?);

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

    // MARK: Func Invoke
    fn expr_func_invoke(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut expr = self.expr_primary()?);

        if let Expression::Identifier(identifier) = &expr.value {
            if self.next_if_eq(&&LexerTokenKind::LParen).is_some() {
                let mut args = vec![];
                let mut end = self.cursor;

                while let Some(token) = self.peek() {
                    dbg!(token);

                    if self.next_if_eq(&&LexerTokenKind::RParen).is_some() {
                        end = self.cursor;
                        break;
                    }

                    if self.next_if_eq(&&LexerTokenKind::Comma).is_some() {
                        continue;
                    }

                    let_expr!(arg = self.expression()?);

                    args.push(arg);
                }

                expr = WithCursor::create_with(
                    expr.start,
                    end,
                    Expression::FunctionCall(Box::from((identifier.to_string(), args))),
                )
            }
        }

        Ok(Some(expr))
    }

    // MARK: Literals/Syntax
    fn expr_primary(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(token = self.next());

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

            LexerTokenKind::LParen => {
                return self.expr_group();
            }

            LexerTokenKind::LBracket => {
                return self.expr_block();
            }

            LexerTokenKind::If => {
                return self.expr_if();
            }

            LexerTokenKind::Match => {
                return self.expr_match();
            }

            LexerTokenKind::EOL | LexerTokenKind::EOF => return Ok(None),

            _ => return Err(ParserErrorKind::UnexpectedToken(token.kind.clone())),
        };

        let_expr!(post = self.expr_range(WithCursor::create_with(token.start, token.end, expr))?);

        Ok(Some(post))
    }

    // MARK: Range
    fn expr_range(
        &mut self,
        lhs: WithCursor<Expression>,
    ) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(peek = self.peek());
        let inclusive = match peek.kind {
            LexerTokenKind::Range => false,
            LexerTokenKind::RangeInclusive => true,
            _ => return Ok(Some(lhs)),
        };

        self.next();
        let_expr!(rhs = self.expression()?);

        Ok(Some(WithCursor::create_with(
            lhs.start,
            rhs.end,
            Expression::Range(Box::from((lhs, rhs, inclusive))),
        )))
    }

    // MARK: Grouping
    fn expr_group(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(mut expr = self.expression()?);

        self.expect_token(&LexerTokenKind::RParen)?;

        expr.value = Expression::Group(Box::from(expr.value));

        Ok(Some(expr))
    }

    // MARK: If
    fn expr_if(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let_expr!(condition = self.expression()?);

        let start = self.cursor;

        let truthy_block = if self.next_if_eq(&&LexerTokenKind::LBracket).is_some() {
            self.stmt_block()?
        } else if self.next_if_eq(&&LexerTokenKind::Colon).is_some() {
            self.parse_inline_block()?
        } else {
            return Err(ParserErrorKind::ExpectedExpression);
        };

        let else_condition = if self.next_if_eq(&&LexerTokenKind::Else).is_some() {
            let start = self.cursor;

            Some(match self.peek().map(|t| t.kind.clone()) {
                Some(LexerTokenKind::LBracket) => {
                    self.next();
                    self.stmt_block()?
                }
                Some(LexerTokenKind::Colon) => {
                    self.next();
                    self.parse_inline_block()?
                }
                _ => {
                    let_expr!(stmt = self.stmt_if()?);
                    WithCursor::create_with(start, self.cursor, vec![stmt])
                }
            })
        } else {
            None
        };

        let if_expr = Expression::If(Box::from((condition, truthy_block, else_condition)));

        Ok(Some(WithCursor::create_with(start, self.cursor, if_expr)))
    }

    fn stmt_if(&mut self) -> ParserResult<Option<Statement>> {
        self.expect_token(&LexerTokenKind::If)?;

        let_expr!(expr = self.expr_if()?);

        Ok(Some(Statement::If(Box::from(expr))))
    }

    // MARK: Match
    fn expr_match(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let start = self.cursor;

        let_expr!(pattern = self.expression()?);

        self.expect_token(&LexerTokenKind::LBracket)?;
        self.expect_terminator()?;

        let mut hash_map = MatchCase::new();

        while let Some(token) = self.peek().cloned() {
            if token.kind == LexerTokenKind::RBracket {
                // self.next();
                break;
            }

            if token.value.is_some() {
                fn to_case(token: &LexerToken) -> ParserResult<WithCursor<Literal>> {
                    Ok(WithCursor::create_with(
                        token.start,
                        token.end,
                        token.to_owned().try_into()?,
                    ))
                }

                let mut cases = vec![];

                // First Case
                cases.push(to_case(token)?);

                self.next();

                while self.next_if_eq(&&LexerTokenKind::Or).is_some() {
                    if let Some(token) = self.next() {
                        cases.push(to_case(token)?)
                    } else {
                        break;
                    }
                }

                self.expect_token(&LexerTokenKind::Arrow)?;

                let_expr!(value = self.expression()?);

                let rc = Rc::new(value);

                for key in cases {
                    hash_map.insert(key, rc.clone());
                }
            }

            self.next();
        }

        Ok(Some(WithCursor::create_with(
            start,
            self.cursor,
            Expression::Match(Box::from((pattern, hash_map))),
        )))
    }

    fn stmt_match(&mut self) -> ParserResult<Option<Statement>> {
        self.expect_token(&LexerTokenKind::Match)?;

        let_expr!(expr = self.expr_match()?);

        Ok(Some(Statement::Match(Box::from(expr))))
    }

    // MARK: Block Parsing
    fn parse_inline_block(&mut self) -> ParserResult<WithCursor<Block>> {
        let Some(next) = self.peek().cloned() else {
            return Err(ParserErrorKind::ExpectedStatement);
        };

        let start = self.cursor;
        let Some(stmt) = self.parse_statement(next)? else {
            return Err(ParserErrorKind::ExpectedStatement);
        };

        self.next();
        Ok(WithCursor::create_with(start, self.cursor, vec![stmt]))
    }

    fn expr_block(&mut self) -> ParserResult<Option<WithCursor<Expression>>> {
        let block = self.stmt_block()?;

        let block = WithCursor::create_with(
            block.start,
            block.end,
            Expression::Block(Box::from(block.value)),
        );

        Ok(Some(block))
    }

    fn stmt_block(&mut self) -> ParserResult<WithCursor<Block>> {
        self.expect_terminator()?;

        let mut block = Block::new();
        let start = self.cursor;

        while let Some(token) = self.peek().cloned() {
            if token.kind == LexerTokenKind::RBracket {
                self.next();
                break;
            }

            if let Some(statement) = self.parse_statement(token)? {
                block.push(statement);
            }

            self.next();
        }

        Ok(WithCursor::create_with(start, self.cursor, block))
    }

    fn expect_token(&mut self, expected: &'a LexerTokenKind) -> ParserResult<&LexerToken> {
        self.expect(&expected).map_err(|found| {
            ParserErrorKind::ExpectedToken(vec![expected.clone()], found.map(|t| t.kind.clone()))
        })
    }

    fn expect_terminator(&mut self) -> ParserResult<Option<&LexerToken>> {
        match self.expect_any(&[&LexerTokenKind::EOL, &LexerTokenKind::EOF]) {
            Ok(found) => Ok(Some(found)),
            Err(None) => Ok(None),
            Err(Some(found)) => Err(ParserErrorKind::ExpectedToken(
                vec![LexerTokenKind::EOL, LexerTokenKind::EOF],
                Some(found.kind.clone()),
            )),
        }
    }
}

// MARK: Comp Error
impl ComponentErrors for Parser<'_> {
    fn fetch_errors(&self) -> &ErrorList {
        &self.errors
    }

    #[cfg(feature = "cli")]
    fn source(&self) -> &crate::error::SourceFile {
        &self.source
    }
}

// MARK: Comp Iter
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