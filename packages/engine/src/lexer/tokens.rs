use std::fmt::Display;

use lang_macro::EnumVariantsTrait;

use crate::{
    cursor::Cursor, error::{EngineErrorKind, EngineResult}, parser::expr::ShellCommand
};

#[repr(u8)]
#[derive(lang_macro::EnumVariants, Debug, PartialEq, Eq, Clone)]
pub enum LexerTokenKind {
    // keywords
    /// `var`           - e.g. var test = 5
    Var,
    /// `fn`            - e.g. fn test(arg: String): Int {...}
    Function,
    /// `for`           - e.g. for i in 0..=5 {...}
    For,
    /// `while`         - e.g. while condition {...}
    While,
    /// `loop`          - e.g. loop {...}
    Loop,
    /// `if`            - e.g. if condition {...} OR if condition: ...
    If,
    /// `else`          
    ///                 - e.g. if condition {...} else if {...} else {...}
    ///                        if condition: ...
    ///                        else: ...
    Else,
    /// `match`         - e.g. match var { pattern => {...}, pattern || pattern => {...} }
    Match,
    /// `break`       
    Break,
    /// `continue`
    Continue,
    /// `return`
    Return,
    /// `in`
    In,
    /// `is`            - note, can be placed before a "not", which would effectively create an "is not" in the tokens list. This should be parsed as a NotEqual
    Is,

    // special
    /// `@include`      - e.g. `@include "path"`
    Include,
    /// `@const`        - e.g. `@const var NAME = value` OR `@const fn test() {...}`
    Const,

    // operator tokens
    /// `=`
    Equal,
    /// `+`
    Plus,
    /// `-`
    Minus,
    /// `/`
    Divide,
    /// `*`
    Multiply,
    /// `==`
    EqualEqual,
    /// `+=`
    PlusEqual,
    /// `-=`
    MinusEqual,
    /// `/=,
    DivideEqual,
    /// `*=`,
    MultiplyEqual,
    /// `!=`,
    NotEqual,
    /// `not` OR `!`
    Not,
    /// `>`
    GreaterThan,
    /// `>=`
    GreaterEqualThan,
    /// `<`
    LesserThan,
    /// `<=`
    LesserEqualThan,
    /// `and` OR `&&`
    And,
    /// `or` OR `||`
    Or,

    // punctuation
    /// `(`
    LParen,
    /// `)`
    RParen,
    /// `{`
    LBracket,
    /// `}`
    RBracket,
    /// `,`
    Comma,
    /// `:`
    Colon,
    /// `=>`
    Arrow,
    /// `..`
    Range,
    /// `..=`
    RangeInclusive,
    /// `\n`
    EOL,
    EOF,

    Identifier,
    String,
    Integer,
    Boolean,
    ShellCommand,
}

impl Display for LexerTokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.variant_name())
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LexerTokenValue {
    Identifier(Box<String>),
    /// `"..."`
    String(Box<String>),
    /// e.g. `1` OR `-5`
    Integer(isize),
    /// `true` OR `false`
    Boolean(bool),
    /// `$cmd arg1 arg2` OR `$cmd(arg1 arg2)`
    ShellCommand(Box<ShellCommand>),
}

impl LexerTokenValue {
    pub fn as_identifier(&self) -> Option<&String> {
        self.try_into().ok()
    }

    pub fn as_string(&self) -> Option<&String> {
        self.try_into().ok()
    }

    pub fn as_integer(&self) -> Option<&isize> {
        self.try_into().ok()
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        self.try_into().ok()
    }

    pub fn as_shell_command(&self) -> Option<&ShellCommand> {
        self.try_into().ok()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LexerToken {
    pub kind: LexerTokenKind,
    pub start: Cursor,
    pub end: Cursor,
    pub value: Option<Box<LexerTokenValue>>,
}

macro_rules! as_value {
    ($self:expr, $token:expr, |$v:ident| => $exp:expr) => {
        $self.value
            .as_ref()
            .and_then(|$v| $exp)
            .ok_or(EngineErrorKind::LiteralExtractionError(
                $token, 
                $self.kind.clone(),
            ))
    };
}

impl LexerToken {

    pub fn as_identifier(&self) -> EngineResult<&String> {
        as_value!(self, LexerTokenKind::Identifier, |v| => v.as_identifier())
    }

    pub fn as_string(&self) -> EngineResult<&String> {
        as_value!(self, LexerTokenKind::String, |v| => v.as_string())
    }

    pub fn as_integer(&self) -> EngineResult<&isize> {
        as_value!(self, LexerTokenKind::Identifier, |v| => v.as_identifier())
    }

    pub fn as_boolean(&self) -> EngineResult<&bool> {
        as_value!(self, LexerTokenKind::Boolean, |v| => v.as_boolean())
    }

    pub fn as_shell_command(&self) -> EngineResult<&ShellCommand> {
        as_value!(self, LexerTokenKind::ShellCommand, |v| => v.as_shell_command())
    }
}

impl PartialEq<LexerTokenKind> for LexerToken {
    fn eq(&self, other: &LexerTokenKind) -> bool {
        &self.kind == other
    }
}

impl PartialEq<LexerToken> for LexerTokenKind {
    fn eq(&self, other: &LexerToken) -> bool {
        self == &other.kind
    }
}

pub type LexerTokenList = Vec<LexerToken>;
