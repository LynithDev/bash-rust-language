#[repr(u16)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // keywords
    Var, // 'var'
    Function, // 'fn'

    // operator tokens
    Equal, // '='
    Plus, // '+'
    Minus, // '-'
    Divide, // '/'
    Multiply, // '*'
    EqualEqual, // '=='
    PlusEqual, // '+='
    MinusEqual, // '-='
    DivideEqual, // '/=,
    MultiplyEqual, // '*=',
    NotEqual, // '!=',
    Not, // '!'

    // literals
    Identifier(String),
    String(String),
    Integer(isize),
    Boolean(bool), // 'true' OR 'false'
    ShellCommand(String, Option<String>), // $"........"

    EOL,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
}

pub type TokenList = Vec<Token>;