#![allow(incomplete_features)]
#![feature(try_blocks)]
#![feature(guard_patterns)]
#![feature(trait_alias)]

#[macro_use]
extern crate log;

use std::path::PathBuf;
use component::ComponentErrors;
use error::EngineResult;
use lexer::Lexer;
use parser::Parser;

pub mod component;
pub mod constants;
pub mod cursor;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod transpiler;

pub struct Engine {}

impl Engine {
    pub fn create() -> Self {
        debug!("created engine");
        Self {}
    }

    pub fn exec_file(&mut self, file: &PathBuf) -> EngineResult<i32> {
        debug!("executing file {file:?}");

        let code = if file.is_file() && (file.is_absolute() || file.is_relative()) {
            std::fs::read_to_string(file).map_err(|_| error::EngineErrorKind::UnknownError)?
        } else {
            return Err(error::EngineErrorKind::UnknownError);
        };

        let mut lexer = Lexer::create(
            &code,
            #[cfg(feature = "cli")]
            Some(file.clone()),
        );
        self.exec_post(&mut lexer)
    }

    pub fn exec(&mut self, code: &str) -> EngineResult<i32> {
        debug!("executing script");

        let mut lexer = Lexer::create(
            code,
            #[cfg(feature = "cli")]
            None,
        );
        self.exec_post(&mut lexer)
    }

    fn exec_post(&mut self, lexer: &mut Lexer) -> EngineResult<i32> {
        #[cfg(feature = "cli")]
        let source_file = lexer.source().clone();
        
        lexer.tokens();  
        if lexer.has_errors() {
            lexer.print_errors();
            return Ok(1);
        }

        let mut parser = Parser::create(
            lexer.tokens(),
            #[cfg(feature = "cli")]
            source_file,
        );

        parser.parse();
        parser.print_errors();

        // let mut transpiler = Transpiler::create(&transpiler::TranspilerTarget::Bash, parser.parse());
        // println!("---START---\n{}\n---END---", transpiler.transpile());


        Ok(0)
    }
}
