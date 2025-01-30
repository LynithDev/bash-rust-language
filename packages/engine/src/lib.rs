#![allow(incomplete_features)]
#![feature(try_blocks)]
#![feature(guard_patterns)]

#[macro_use] extern crate log;

use std::path::PathBuf;

use component::ComponentErrors;
use error::EngineResult;
use lexer::Lexer;

pub mod lexer;
pub mod parser;
pub mod error;
pub mod constants;
pub mod component;
mod cursor;

pub use cursor::Cursor;

#[derive(Default)]
pub struct Engine {

}

impl Engine {
    pub fn create() -> Self {
        debug!("created engine");
        Self {}
    }

    pub fn exec_file(&mut self, file: &PathBuf) -> EngineResult<i32> {
        debug!("executing file {file:?}");

        let code = if file.is_file() && (file.is_absolute() || file.is_relative()) {
            std::fs::read_to_string(file).map_err(|_| error::EngineError::UnknownError)?
        } else {
            return Err(error::EngineError::UnknownError);
        };

        let mut lexer = Lexer::create(&code, #[cfg(feature = "cli")] Some(file.clone()));
        println!("{:#?}", lexer.tokenize());

        lexer.print_errors();

        Ok(0)
    }

    pub fn exec(&mut self, code: &str) -> EngineResult<i32> {
        debug!("executing script");

        let mut lexer = Lexer::create(code, #[cfg(feature = "cli")] None);
        println!("{:#?}", lexer.tokenize());

        lexer.print_errors();

        Ok(0)
    }
}