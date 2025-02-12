use crate::{parse, ast, as_stmt_kind, parseable, parser::expr::IfExpr};

ast!(IfStmt(IfExpr));
as_stmt_kind!(IfStmt = If);

parseable! {
    IfStmt = |parser| {
        parser.expect_token(&LexerTokenKind::If)?;

        parse!(parser, expr = IfExpr);

        Ok(Some(IfStmt(expr)))
    }
}