use boolean::BooleanLiteral;
use integer::IntegerLiteral;
use string::StringLiteral;

pub mod boolean;
pub mod identifier;
pub mod string;
pub mod integer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Literal {
    String(StringLiteral),
    Integer(IntegerLiteral),
    Boolean(BooleanLiteral),
}