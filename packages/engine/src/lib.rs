#![allow(incomplete_features)]
#![feature(try_blocks)]
#![feature(guard_patterns)]

#[macro_use] extern crate log;

use std::path::PathBuf;

use error::EngineResult;
use lexer::Lexer;

pub mod lexer;
pub mod error;
pub mod constants;
mod cursor;
mod utils;

pub use cursor::Cursor;

#[derive(Default)]
pub struct Engine {

}

impl Engine {
    pub fn create() -> Self {
        Self {

        }
    }

    pub fn exec_file(&mut self, file: &PathBuf) -> EngineResult<i32> {
        let code = if file.is_file() && (file.is_absolute() || file.is_relative()) {
            std::fs::read_to_string(file).map_err(|_| error::EngineError::UnknownError)?
        } else {
            return Err(error::EngineError::UnknownError);
        };

        let mut lexer = Lexer::create(&code, Some(file.clone()));
        println!("{:#?}", lexer.tokenize());

        lexer.print_errors();

        Ok(0)
    }

    pub fn exec(&mut self, code: &str) -> EngineResult<i32> {
        let mut lexer = Lexer::create(code, None);
        
        println!("{:#?}", lexer.tokenize());

        lexer.print_errors();

        Ok(0)
    }
}