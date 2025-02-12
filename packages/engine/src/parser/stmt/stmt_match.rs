use crate::{as_stmt_kind, ast, ok_or_none, parseable, parser::expr::MatchExpr};

ast!(MatchStmt(MatchExpr));
as_stmt_kind!(MatchStmt = Match);

parseable! {
    MatchStmt = |parser| {
        parser.expect_token(&LexerTokenKind::Match)?;

        let expr = ok_or_none!(MatchExpr::parse(parser)?);

        Ok(Some(MatchStmt(expr)))
    }
}