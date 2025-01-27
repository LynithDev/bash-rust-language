#![allow(incomplete_features)]
#![feature(try_blocks)]
#![feature(guard_patterns)]

#[macro_use] extern crate log;

use error::EngineResult;
use lexer::Lexer;

pub mod lexer;
pub mod error;
pub mod constants;

#[derive(Default)]
pub struct Engine {

}

impl Engine {
    pub fn create() -> Self {
        Self {

        }
    }

    pub fn exec(&mut self, code: &str) -> EngineResult<i32> {
        let mut lexer = Lexer::create(code);

        println!("{:#?}", lexer.tokenize());

        Ok(0)
    }
}