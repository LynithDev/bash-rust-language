mod targets;
pub mod error;

use error::TranspilerResult;
use crate::parser::ast::{Expression, ProgramTree, Statement};
pub use targets::bash::BashTranspiler;

// MARK: Transpilers
#[derive(Debug, PartialEq, Default)]
pub enum TranspilerTarget {
    #[default]
    Bash,
}

impl TranspilerTarget {
    fn get_impl(&self) -> Box<dyn TranspilerImpl> {
        Box::from(match self {
            Self::Bash => BashTranspiler::default(),
        })
    }
}

// MARK: Main
pub struct Transpiler<'a> {
    tree: &'a ProgramTree,
    inner: Box<dyn TranspilerImpl<'a>>,
    out: String
}

impl<'a> Transpiler<'a> {
    pub fn create(target: &'a TranspilerTarget, tree: &'a ProgramTree) -> Self {
        Self {
            tree,
            inner: target.get_impl(),
            out: String::new()
        }
    }
    
    pub fn transpile(&mut self) -> &String {
        if !self.out.is_empty() {
            return &self.out;
        }
        
        for statement in self.tree {
            match self.inner.transpile_stmt(statement) {
                Ok(out) => self.out.push_str(out.as_str()),
                Err(err) => error!("{err:#?}")
            }
        }
        
        &self.out
    }
}

// MARK: Context
#[derive(Default)]
pub struct TranspilerContext {
    pub scope_depth: i8,
    pub in_loop: bool,
}

// MARK: Definition
pub trait TranspilerImpl<'a> {
    fn ctx(&self) -> &TranspilerContext;

    fn transpile_stmt(&self, statement: &'a Statement) -> TranspilerResult<String>;
    fn transpile_expr(&self, expression: &'a Expression) -> TranspilerResult<String>;
}