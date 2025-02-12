use crate::{as_stmt_kind, ast, parse, parseable, parser::expr::MatchExpr};

ast!(MatchStmt(MatchExpr));
as_stmt_kind!(MatchStmt = Match);

parseable! {
    MatchStmt = |parser| {
        parser.expect_token(&LexerTokenKind::Match)?;

        parse!(parser, expr = MatchExpr);

        Ok(Some(MatchStmt(expr)))
    }
}