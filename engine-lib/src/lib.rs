#![allow(incomplete_features)]
#![feature(try_blocks)]
#![feature(guard_patterns)]

#[macro_use] extern crate log;

use error::EngineResult;
use lexer::Lexer;

pub mod lexer;
pub mod error;
pub mod constants;
mod cursor;

pub use cursor::Cursor;

#[derive(Default)]
pub struct Engine {

}

impl Engine {
    pub fn create() -> Self {
        Self {

        }
    }

    pub fn exec(&mut self, code: &str) -> EngineResult<i32> {
        let mut lexer = Lexer::create(code, None);
        debug!("created lexer");

        println!("{:#?}", lexer.tokenize());

        lexer.fetch_errors().iter().for_each(|e| println!("{e}"));

        Ok(0)
    }
}