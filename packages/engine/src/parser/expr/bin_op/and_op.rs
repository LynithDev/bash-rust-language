use crate::{ast, parse_bin_op, parser::expr::Expression, to_expr_kind};

ast!(And(Expression, Expression));
to_expr_kind!(And);

parse_bin_op! {
    And = |parser| {
        left = {
            parser.expr_cmp_equality()
        }

        right = {
            parser.expr_cmp_equality()
        }
    }
}