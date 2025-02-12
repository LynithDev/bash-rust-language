use crate::{to_expr_kind, ast, parse, parse_bin_op, parseable, parser::{ast::ToExpressionKind, expr::{bin_op::BinOp, Expression, Or}}};

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
        parse!(parser, lhs = Or);
        parse!(parser, rhs = Or);
        Ok(Some(And(lhs.as_expr_kind(), rhs.as_expr_kind())))
    }
}

// let_expr!(mut lhs = self.expr_logic_and()?);

// while let Some(token_or) = self.next_if_eq(&&LexerTokenKind::Or) {
//     let_expr!(rhs = self.expr_logic_and()?);

//     let operator: LogicalOperator = LogicalOperator::Or;

//     lhs = WithCursor::create_with(
//         lhs.start,
//         rhs.end,
//         ExpressionKind::Logic(Box::from((
//             lhs,
//             WithCursor::create_with(token_or.start, token_or.end, operator),
//             rhs,
//         ))),
//     );
// }

// Ok(Some(lhs))