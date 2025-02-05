use std::{collections::HashMap, rc::Rc};

use crate::{component::ComponentIter, lexer::tokens::LexerTokenKind, parser::ast::Parse};

use super::{literal::Literal, Expression, ExpressionKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchExpr(pub Expression, pub HashMap<Literal, Rc<Expression>>);

impl Parse<ExpressionKind> for MatchExpr {
    fn parse(parser: &mut crate::parser::Parser) -> crate::parser::ParserResult<ExpressionKind> {
        let start = parser.cursor;

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

        Ok(ExpressionKind::Match((pattern, hash_map)))
    }
}