use crate::{parse, ast, as_stmt, parseable, parser::expr::IfExpr};

ast!(IfStmt(IfExpr));
as_stmt!(IfStmt = If);

parseable! {
    IfStmt = |parser| {
        parser.expect_token(&LexerTokenKind::If)?;

        parse!(parser, expr = IfExpr);

        Ok(Some(IfStmt(expr)))
    }
}