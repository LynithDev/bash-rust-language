use std::{collections::HashMap, rc::Rc};

use crate::{as_expr, ast, parseable};

use super::{literal::Literal, Expression};

ast!(MatchExpr(Expression, HashMap<Literal, Rc<Expression>>));
as_expr!(MatchExpr = Match);

parseable! {
    MatchExpr = |parser| {
        let pattern = parser.expression()?;

        parser.expect_token(&LexerTokenKind::LBracket)?;
        parser.expect_terminator()?;

        let mut hash_map = HashMap::new();

        while let Some(token) = parser.peek().cloned() {
            if token.kind == LexerTokenKind::RBracket {
                break;
            }

            if token.value.is_some() {
                let mut cases: Vec<Literal> = vec![];

                // First Case
                cases.push(token.to_owned().try_into()?);

                parser.next();

                while parser.next_if_eq(&&LexerTokenKind::Or).is_some() {
                    if let Some(token) = parser.next() {
                        cases.push(token.to_owned().try_into()?)
                    } else {
                        break;
                    }
                }

                parser.expect_token(&LexerTokenKind::Arrow)?;

                let value = parser.expression()?;

                let rc = Rc::new(value);

                for key in cases {
                    hash_map.insert(key, rc.clone());
                }
            }

            parser.next();
        }

        Ok(Some(MatchExpr(pattern, hash_map)))
    }
}
