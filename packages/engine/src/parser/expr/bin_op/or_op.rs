use crate::{ast, parse_bin_op, parser::expr::Expression, to_expr_kind};

ast!(Or(Expression, Expression));
to_expr_kind!(Or);

parse_bin_op! {
    Or = |parser| {
        left = {
            parser.expr_logic_and()
        }

        right = {
            parser.expr_logic_and()
        }
    }
}