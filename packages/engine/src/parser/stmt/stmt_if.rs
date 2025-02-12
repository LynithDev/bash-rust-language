use crate::{as_stmt_kind, ast, ok_or_none, parseable, parser::expr::IfExpr};

ast!(IfStmt(IfExpr));
as_stmt_kind!(IfStmt = If);

parseable! {
    IfStmt = |parser| {
        parser.expect_token(&LexerTokenKind::If)?;

        let expr = ok_or_none!(IfExpr::parse(parser)?);

        Ok(Some(IfStmt(expr)))
    }
}