use crate::{ast, ok_or_none, parseable, parser::stmt::Statement, to_expr_kind};

ast!(Block(Vec<Statement>)); 
to_expr_kind!(Block);

parseable! {
    Block = |parser| {
        parser.expect_terminator()?;

        let mut statements = vec![];

        while let Some(token) = parser.peek().cloned() {
            if token.kind == LexerTokenKind::RBracket {
                parser.next();
                break;
            }

            let statement = ok_or_none!(parser.statement(token)?);
            statements.push(statement);

            parser.next();
        }

        Ok(Some(Block(statements)))
    }
}
