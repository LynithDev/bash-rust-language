use crate::cursor::CursorTuple;

#[repr(u8)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
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
    LParam,
    /// `)`
    RParam,
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



    // literals & types
    Identifier(Box<String>),
    /// `"..."`
    String(Box<String>),
    /// e.g. `1` OR `-5`
    Integer(isize),
    /// `true` OR `false`
    Boolean(bool),
    /// `$cmd arg1 arg2` OR `$cmd(arg1 arg2)`
    ShellCommand(Box<(String, Option<String>)>),


    /// `\n`
    EOL,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub start: CursorTuple,
    pub end: CursorTuple,
}

pub type TokenList = Vec<Token>;
