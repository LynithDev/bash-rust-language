#![allow(incomplete_features)]
#![feature(try_blocks)]
#![feature(guard_patterns)]
#![feature(trait_alias)]

#[macro_use]
extern crate log;

pub mod component;
pub mod constants;
pub mod cursor;
pub mod error;
pub mod lexer;
pub mod parser;
pub mod transpiler;
pub mod engine;

#[macro_export]
macro_rules! ok_or_none {
    ($expr:expr) => {
        match $expr {
            Some(val) => val,
            None => return Ok(None),
        }
    };
}