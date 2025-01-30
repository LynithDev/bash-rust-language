use std::{iter::Peekable, slice::Iter};

use ast::{ProgramTree, Statement};
use error::ParserResult;

use crate::{
    component::{ComponentErrors, ComponentIter},
    error::{EngineError, ErrorList},
    lexer::tokens::{LexerToken, LexerTokenKind, LexerTokenList},
    Cursor,
};

pub use error::{ParserError, ParserErrorKind};

pub mod ast;
mod error;

pub struct Parser<'a> {
    tokens: Peekable<Iter<'a, LexerToken>>,
    cursor: Cursor,
    errors: ErrorList,

    #[cfg(feature = "cli")]
    source: crate::error::SourceFile,
}

impl<'a> Parser<'a> {
    pub fn create(
        tokens: &'a LexerTokenList,
        #[cfg(feature = "cli")] source: crate::error::SourceFile,
    ) -> Self {
        let parser = Self {
            tokens: tokens.iter().peekable(),
            cursor: Cursor::create(),
            errors: ErrorList::new(),

            #[cfg(feature = "cli")]
            source,
        };

        debug!("created parser");
        parser
    }

    pub fn parse(&mut self) -> ProgramTree {
        let mut statements = ProgramTree::new();

        while let Some(token) = self.next() {
            match self.parse_statement(token) {
                Ok(Some(statement)) => statements.push(statement),
                Err(err) => {
                    self.errors.push(EngineError::ParserError(ParserError {
                        start: token.start,
                        end: token.end,
                        kind: err,
                        #[cfg(feature = "cli")]
                        source_file: self.get_source_sliced(token.start, token.end),
                    }));
                }
                _ => {}
            }
        }

        statements
    }

    fn parse_statement(&mut self, token: &LexerToken) -> ParserResult<Option<Statement>> {
        Ok(None)
    }
}

impl ComponentErrors for Parser<'_> {
    fn fetch_errors(&self) -> &ErrorList {
        &self.errors
    }

    #[cfg(feature = "cli")]
    fn source(&self) -> &crate::error::SourceFile {
        &self.source
    }
}

impl<'a> ComponentIter<'a, &'a LexerTokenKind, &'a LexerToken, Iter<'a, LexerToken>>
    for Parser<'a>
{
    fn get_iter(&mut self) -> &mut Peekable<Iter<'a, LexerToken>> {
        &mut self.tokens
    }

    fn cursor_next(&mut self, item: &&LexerToken) {
        self.cursor.clone_from(&item.start);
    }
}
