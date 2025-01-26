use std::{iter::Peekable, str::Chars};

use tokens::{Token, TokenList, TokenType};

use crate::error::{EngineError, EngineResult};

pub mod tokens;

#[derive(Debug, Clone)]
struct Cursor {
    pub line: usize,
    pub col: usize,
}

impl Cursor {
    pub fn new() -> Self {
        Self {
            line: 1,
            col: 0,
        }
    }

    pub fn new_line(&mut self) {
        self.line += 1;
        self.col = 0;
    } 

    pub fn next(&mut self) {
        self.col += 1;
    }

    pub fn string(&mut self, string: &str) {
        for char in string.chars() {
            self.char(&char);
        }
    }

    pub fn char(&mut self, char: &char) {
        if char == &'\n' {
            self.new_line();
        } else {
            self.next();
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum LexerError {
    #[error("unexpected end of input")]
    UnexpectedEnd
}

type CharIterator<'a> = Peekable<Chars<'a>>;

pub fn tokenize(code: String) -> EngineResult<TokenList> {
    let code = code.trim().to_string();

    let mut tokens = TokenList::new();
    let mut chars = code.chars().peekable();
    
    let mut cursor = Cursor::new();

    let result: EngineResult<()> = try {
        loop {
            let Some(char) = chars.next() else {
                // No more characters, stop the loop
                break;
            };
    
            cursor.next();

            tokens.push(Token {
                col: cursor.col,
                line: cursor.line,
                token_type: match char {
                    _ if SEPARATORS.contains(&char) => continue,

                    _ if TERMINATORS.contains(&char) => {
                        cursor.new_line();
                        TokenType::EOL
                    },
                    
                    '=' => if_next_is(&mut cursor, &mut chars, &'=', TokenType::EqualEqual, TokenType::Equal),
                    '+' => if_next_is(&mut cursor, &mut chars, &'=', TokenType::PlusEqual, TokenType::Plus),
                    '-' => if_next_is(&mut cursor, &mut chars, &'=', TokenType::MinusEqual, TokenType::Minus),
                    '/' => if_next_is(&mut cursor, &mut chars, &'=', TokenType::DivideEqual, TokenType::Divide),
                    '*' => if_next_is(&mut cursor, &mut chars, &'=', TokenType::MultiplyEqual, TokenType::Multiply),
                    '!' => if_next_is(&mut cursor, &mut chars, &'=', TokenType::NotEqual, TokenType::Not),
                    
                    '"' => TokenType::String(string(&mut cursor, &mut chars)?),

                    '$' => {
                        let cmd_name = collect_until(&mut cursor, &mut chars, &[SEPARATORS, &['(']].concat())?;
                        
                        let arguments = match chars.peek() {
                            Some('(') => {
                                chars.next(); // consume (
                                cursor.next();
                                collect_until(&mut cursor, &mut chars, &[')']).ok()
                            },
                            Some(char) if SEPARATORS.contains(char) => {
                                chars.next(); // consume the character
                                cursor.next();
                                collect_until(&mut cursor, &mut chars, TERMINATORS).ok()
                            },
                            _ => None,
                        };

                        if arguments.is_some() {
                            // Consume ')'
                            next_if_eq(&mut cursor, &mut chars, &')');
                        }

                        TokenType::ShellCommand(cmd_name, arguments)
                    }
                    
                    _ => {
                        let keyword = collect_until(&mut cursor, &mut chars, WHITESPACE).unwrap_or_default();
                        let keyword = format!("{char}{keyword}");

                        match keyword.as_str() {
                            "var" => TokenType::Var,
                            "fn" => TokenType::Function,
                            "true" => TokenType::Boolean(true),
                            "false" => TokenType::Boolean(false),

                            _ => {
                                // If we haven't found a token, it most likely 
                                // means that its an identifier of some sort.

                                // TODO: Figure out better approach for this
                                if let Ok(int) = keyword.parse::<isize>() {
                                    TokenType::Integer(int)
                                } else {
                                    TokenType::Identifier(keyword)
                                }
                            }
                        }
                    },
                }
            });
        }
    };

    if let Err(err) = result {
        error!("error during tokenization: {err}");
    }

    tokens.push(Token {
        token_type: TokenType::EOL,
        col: cursor.col + 1,
        line: cursor.line,
    });

    Ok(tokens)
}

fn string(cursor: &mut Cursor, chars: &mut CharIterator) -> EngineResult<String> {
    let res = collect_until(cursor, chars, &['"'])?;

    cursor.next();
    chars.next();

    Ok(res)
}

fn collect_until(cursor: &mut Cursor, chars: &mut CharIterator, term: &[char]) -> EngineResult<String> {
    let mut collector = String::new();
    
    loop {
        let Some(char) = chars.peek() else {
            break;
        };

        if term.contains(char) {
            break;
        }
        
        let Some(next) = chars.next() else {
            return Err(EngineError::LexerError(LexerError::UnexpectedEnd))
        };
        
        collector.push(next);
    }

    cursor.string(&collector);
    Ok(collector)
}


fn if_next_is<R>(cursor: &mut Cursor, chars: &mut CharIterator, char: &char, r#true: R, r#false: R) -> R {
    if next_if_eq(cursor, chars, char) {
        cursor.char(char);
        r#true
    } else {
        r#false
    }
}

fn next_if_eq(cursor: &mut Cursor, chars: &mut CharIterator, char: &char) -> bool {
    if chars.next_if_eq(char).is_some() {
        cursor.char(char);
        true
    } else {
        false
    }
}


const SEPARATORS: &[char] = &[' ', '\t', '\0'];
const WHITESPACE: &[char] = &[' ', '\t', '\n', '\r', '\0'];
const TERMINATORS: &[char] = &['\n', '\r'];