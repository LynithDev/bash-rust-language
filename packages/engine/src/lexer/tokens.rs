use std::fmt::Display;

use lang_macro::EnumVariantsTrait;

use crate::{
    error::{EngineErrorKind, EngineResult},
    Cursor,
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
pub enum LexerLiteral {
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

impl LexerLiteral {
    pub fn as_identifier(&self) -> Option<&String> {
        match self {
            Self::Identifier(identifier) => Some(identifier),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(string) => Some(string),
            _ => None,
        }
    }

    pub fn as_integer(&self) -> Option<&isize> {
        match self {
            Self::Integer(int) => Some(int),
            _ => None,
        }
    }

    pub fn as_boolean(&self) -> Option<&bool> {
        match self {
            Self::Boolean(bool) => Some(bool),
            _ => None,
        }
    }

    pub fn as_shell_command(&self) -> Option<&ShellCommand> {
        match self {
            Self::ShellCommand(cmd) => Some(cmd),
            _ => None,
        }
    }
}

pub type ShellCommand = (String, Option<String>);

#[derive(Debug, PartialEq, Clone)]
pub struct LexerToken {
    pub kind: LexerTokenKind,
    pub start: Cursor,
    pub end: Cursor,
    pub value: Option<Box<LexerLiteral>>,
}

impl LexerToken {
    pub fn as_identifier(&self) -> EngineResult<&String> {
        self.value
            .as_ref()
            .and_then(|v| v.as_identifier())
            .ok_or(EngineErrorKind::LiteralExtractionError(
                LexerTokenKind::Identifier, 
                self.kind.clone(),
            ))
    }

    pub fn as_string(&self) -> EngineResult<&String> {
        self.value
            .as_ref()
            .and_then(|v| v.as_string())
            .ok_or(EngineErrorKind::LiteralExtractionError(
                LexerTokenKind::String,
                self.kind.clone(),
            ))
    }

    pub fn as_integer(&self) -> EngineResult<&isize> {
        self.value
            .as_ref()
            .and_then(|v| v.as_integer())
            .ok_or(EngineErrorKind::LiteralExtractionError(
                LexerTokenKind::Integer,
                self.kind.clone(),
            ))
    }

    pub fn as_boolean(&self) -> EngineResult<&bool> {
        self.value
            .as_ref()
            .and_then(|v| v.as_boolean())
            .ok_or(EngineErrorKind::LiteralExtractionError(
                LexerTokenKind::Boolean,
                self.kind.clone(),
            ))
    }

    pub fn as_shell_command(&self) -> EngineResult<&ShellCommand> {
        self.value
            .as_ref()
            .and_then(|v| v.as_shell_command())
            .ok_or(EngineErrorKind::LiteralExtractionError(
                LexerTokenKind::ShellCommand,
                self.kind.clone(),
            ))
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
