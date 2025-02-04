#![allow(incomplete_features)]
#![feature(try_blocks)]
#![feature(guard_patterns)]

#[macro_use]
extern crate log;

use std::path::PathBuf;

use component::ComponentErrors;
use error::{EngineResult, SourceFile};

pub use lexer::Lexer;
pub use parser::Parser;
use transpiler::{Transpiler, TranspilerTarget};

pub mod component;
pub mod constants;
pub mod cursor;
pub mod error;

pub mod lexer;
pub mod parser;
pub mod transpiler;

#[derive(Default)]
pub struct Engine {
    target: TranspilerTarget,
}

impl Engine {
    pub fn create(target: TranspilerTarget) -> Self {
        Self {
            target
        }
    }

    pub fn exec_file(&mut self, file: &PathBuf) -> EngineResult<i32> {
        debug!("attempting to read file {file:?}");

        if !file.is_file() {
            return Err(error::EngineErrorKind::ExpectedFileError);
        }

        let code = std::fs::read_to_string(file)?;
        
        let path = file
            .canonicalize()
            .unwrap_or(file.clone())
            .to_string_lossy()
            .to_string();

        self.exec_source_file(SourceFile::from(code, Some(path)))
    }

    pub fn exec(&mut self, code: &str) -> EngineResult<i32> {
        self.exec_source_file(SourceFile::from(code.to_string(), None))
    }

    pub fn exec_source_file(&mut self, source_file: error::SourceFile) -> EngineResult<i32> {
        let mut lexer = Lexer::create(&source_file);

        lexer.tokens();
        if lexer.has_errors() {
            lexer.print_errors();
            return Err(error::EngineErrorKind::ExecError);
        }

        let mut parser = Parser::create(lexer.tokens(), &source_file);
        parser.parse();
        if parser.has_errors() {
            parser.print_errors();
            return Err(error::EngineErrorKind::ExecError);
        }

        let mut transpiler = Transpiler::create(&self.target, parser.parse());
        println!("---START---\n{}\n---END---", transpiler.transpile());

        Ok(0)
    }
}
