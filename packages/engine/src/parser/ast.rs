use super::{stmt::Statement, Parser, ParserResult};

pub type ProgramTree = Vec<Statement>;

pub trait Parse<T> {
    fn parse(parser: &mut Parser) -> ParserResult<T>;
}