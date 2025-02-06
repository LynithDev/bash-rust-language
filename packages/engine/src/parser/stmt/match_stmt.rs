use crate::{as_stmt, ast, parse, parseable, parser::expr::MatchExpr};

ast!(MatchStmt(MatchExpr));
as_stmt!(MatchStmt = Match);

parseable! {
    MatchStmt = |parser| {
        parser.expect_token(&LexerTokenKind::Match)?;

        parse!(parser, expr = MatchExpr);

        Ok(Some(MatchStmt(expr)))
    }
}