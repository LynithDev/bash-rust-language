#![feature(try_blocks)]

#[macro_use] extern crate log;

use error::EngineResult;

pub mod lexer;
pub mod error;

#[derive(Default)]
pub struct Engine {}

impl Engine {
    pub fn create() -> Self {
        Self {
            
        }
    }

    pub fn exec(&mut self, code: String) -> EngineResult<i32> {
        let tokens = lexer::tokenize(code)?;

        println!("{:#?}", tokens);

        Ok(0)
    }

    #[inline]
    pub fn exec_str(&mut self, code: &str) -> EngineResult<i32> {
        self.exec(code.to_string())
    }
}