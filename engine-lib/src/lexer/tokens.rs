#[repr(u16)]
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // keywords
    Var, // 'var'                   - e.g. var test = 5
    Function, // 'fn'               - e.g. fn test(arg: String): Int {...}
    For, // 'for'                   - e.g. for i = 0, i < 5, i += 1 { ... }     OR  for i in 0..5 {...}     OR  for {...}
    If, // 'if'                     - e.g. if condition {...}
    Else, // 'else'                 - e.g. if condition {...} else if {...} else {...}
    Match, // 'match'               - e.g. match var { pattern => {...}, pattern || pattern => {...} }
    Break, // 'break'       
    Continue, // 'continue'
    Return, // 'return'
    In, // 'in'
    
    
    // special
    Include, // '@include'          - e.g. '@include "path"'
    Const, // '@const'              - e.g. '@const var NAME = value' OR '@const fn test() {...}'
    


    // operator tokens
    Equal, // 'is' OR '='
    Plus, // '+'
    Minus, // '-'
    Divide, // '/'
    Multiply, // '*'
    EqualEqual, // '=='
    PlusEqual, // '+='
    MinusEqual, // '-='
    DivideEqual, // '/=,
    MultiplyEqual, // '*=',
    NotEqual, // 'is not' OR '!=',
    Not, // 'not' OR '!'
    And, // 'and' OR '&&'
    Or, // 'or' OR '||'

    // punctuation
    LParam, // '('
    RParam, // ')'
    LBracket, // '{'
    RBracket, // '}'
    Comma, // ','
    Colon, // ':'

    // literals & types
    Identifier(String), 
    String(String), // '"..."'
    Integer(isize), // e.g. '1' OR '-5'
    Boolean(bool), // 'true' OR 'false'
    ShellCommand(String, Option<String>), // '$cmd arg1 arg2' OR '$cmd(arg1 arg2)'

    EOL,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: usize,
    pub col: usize,
}

pub type TokenList = Vec<Token>;