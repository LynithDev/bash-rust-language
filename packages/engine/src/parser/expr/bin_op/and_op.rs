use crate::{ast, parse_bin_op, parse_expr, parseable, parser::expr::{Expression, Or}, to_expr_kind};

ast!(And(Expression, Expression));
to_expr_kind!(And);

parse_bin_op! {
    And = |parser| {
        left = {
            todo!();
        }

        right = {
            todo!();
        }
    }
}

parseable! {
    And = |parser| {
        parse_expr!(parser, lhs = Or);
        parse_expr!(parser, rhs = Or);
        Ok(Some(And(lhs, rhs)))
    }
}

    // fn expr_logic_and(&mut self) -> ParserResult<Option<WithCursor<ExpressionKind>>> {
    //     let_expr!(mut lhs = self.expr_cmp_equality()?);

    //     while let Some(token_and) = self.next_if_eq(&&LexerTokenKind::And) {
    //         let_expr!(rhs = self.expr_cmp_equality()?);

    //         let operator: LogicalOperator = LogicalOperator::And;

    //         lhs = WithCursor::create_with(
    //             lhs.start,
    //             rhs.end,
    //             ExpressionKind::Logic(Box::from((
    //                 lhs,
    //                 WithCursor::create_with(token_and.start, token_and.end, operator),
    //                 rhs,
    //             ))),
    //         );
    //     }

    //     Ok(Some(lhs))
    // }