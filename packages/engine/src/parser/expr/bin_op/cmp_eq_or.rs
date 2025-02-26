use crate::{ast, parse_bin_op, parser::expr::Expression, to_expr_kind};

ast!(CmpEquality(Expression, Expression));
to_expr_kind!(CmpEquality);

parse_bin_op! {
    CmpEquality = |parser| {
        left = {
            parser.expr_logic_and()
        }

        right = {
            parser.expr_logic_and()
        }
    }
}