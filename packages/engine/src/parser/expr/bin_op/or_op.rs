use crate::{to_expr_kind, ast, parse, parse_bin_op, parseable, parser::{ast::Parse, expr::{bin_op::{and_op::And, BinOp}, Expression}}};

ast!(Or(Expression, Expression));
to_expr_kind!(Or);

parse_bin_op! {
    Or = |parser| {
        left = {
            And::parse(parser)
        }

        right = {
            parser.expr_logic_and()
        }
    }
}

parseable! {
    Or = |parser| {
        let lhs = Or::parse_left(parser);
        parse!(parser, rhs = And);
        Ok(Some(Or(lhs, rhs)))
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