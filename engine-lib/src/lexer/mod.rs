use std::{iter::Peekable, path::PathBuf, str::Chars};

use error::LexerResult;
use tokens::{Token, TokenList, TokenType};

use crate::{
    constants::{MAX_I32_LEN, MAX_I64_LEN}, Cursor, error::{EngineError, ErrorList}
};

pub use error::{LexerError, LexerErrorKind};

mod error;
pub mod tokens;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    errors: ErrorList,
    cursor: Cursor,
    tokens: TokenList,
    max_int_len: u8,
    
    #[cfg(feature = "cli")]
    source: &'a str,

    #[cfg(feature = "cli")]
    path: Option<PathBuf>,
}

impl<'a> Lexer<'a> {
    pub fn create(input: &'a str, path: Option<PathBuf>) -> Self {
        Self::create_bits(input, path, MAX_I64_LEN)
    }

    pub fn create_32b(input: &'a str, path: Option<PathBuf>) -> Self {
        Self::create_bits(input, path, MAX_I32_LEN)
    }

    pub fn create_bits(input: &'a str, path: Option<PathBuf>, max_int_len: u8) -> Self {
        Self {
            chars: input.trim().chars().peekable(),
            errors: ErrorList::new(),
            cursor: Cursor::create(),
            tokens: TokenList::new(),
            max_int_len,

            #[cfg(feature = "cli")]
            source: input,

            #[cfg(feature = "cli")]
            path,
        }
    }

    pub fn tokenize(&mut self) -> &TokenList {
        while self.peek().is_some() {
            let start = self.cursor;

            if let Some(char) = self.next() {
                match self.scan_char(&char) {
                    Ok(Some(token_type)) => self.add_token(token_type, start),
                    Err(err) => {
                        self.errors.push(EngineError::LexerError(LexerError {
                            #[cfg(feature = "cli")]
                            source_file: self.get_source_file(start, self.cursor),
                            start,
                            end: self.cursor,
                            kind: err,

                        }));
                    },
                    _ => {}
                }
            }
        }

        let start = self.cursor;
        self.cursor.next_line();
        self.add_token(TokenType::EOL, start);

        &self.tokens
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn fetch_errors(&self) -> &ErrorList {
        &self.errors
    }

    #[cfg(feature = "cli")]
    fn get_source_file(&self, start: Cursor, end: Cursor) -> error::SourceFile {
        let path = self.path.clone();

        let start_index = dbg!(start).index() as usize;
        let end_index = dbg!(end).index() as usize;

        let source = &self.source[start_index..end_index];

        Box::from((path, source.to_string()))
    }

    fn scan_char(&mut self, char: &char) -> LexerResult<Option<TokenType>> {
        macro_rules! check_double {
            ($single_type:expr, $double:tt, $double_type:expr) => {
                if self.next_if_eq(&$double).is_some() {
                    $double_type
                } else {
                    $single_type
                }
            };
        }

        macro_rules! double {
            ($a:tt, $b:tt) => {
                $a if self.peek_is(&$b)
            };
        }

        use TokenType::*;

        Ok(Some(match char {
            ' ' => return Ok(None),
            '\n' => EOL,

            '=' => check_double!(Equal, '=', EqualEqual),
            '+' => check_double!(Plus, '=', PlusEqual),
            '-' => match self.peek() {
                Some(&'=') => {
                    self.next();
                    MinusEqual
                }
                Some(&('0'..='9')) => {
                    let Some(char) = self.next() else {
                        return Ok(None);
                    };

                    Integer(-self.eat_number(char)?)
                }
                _ => Minus,
            },
            '*' => check_double!(Multiply, '=', MultiplyEqual),

            '/' => match self.peek() {
                Some(&'/') => return self.consume_single_line_comment(),
                Some(&'*') => return self.consume_multi_line_comment(),
                Some(&'=') => {
                    self.next();
                    DivideEqual
                }
                _ => Divide,
            },

            '!' => check_double!(Not, '=', NotEqual),
            '<' => check_double!(LesserThan, '=', LesserEqualThan),
            '>' => check_double!(GreaterThan, '=', GreaterEqualThan),
            double!('&', '&') => And,
            double!('|', '|') => Or,

            '(' => LParam,
            ')' => RParam,
            '{' => LBracket,
            '}' => RBracket,
            ',' => Comma,
            ':' => Colon,

            '0'..='9' => Integer(self.eat_number(*char)?),
            '"' => self.consume_string()?,

            '$' => self.consume_shell_command()?,

            char => {
                let consumed = self.eat_word().unwrap_or_default();

                let word = format!("{char}{consumed}");
                match word.as_str() {
                    "var" => Var,
                    "fn" => Function,
                    "for" => For,
                    "if" => If,
                    "else" => Else,
                    "match" => Match,
                    "break" => Break,
                    "continue" => Continue,
                    "return" => Return,
                    "in" => In,
                    "is" => Is,

                    "@include" => Include,
                    "@const" => Const,

                    "not" => Not,
                    "and" => And,
                    "or" => Or,

                    "true" => Boolean(true),
                    "false" => Boolean(false),

                    _ if Self::is_valid_identifier(char) => Identifier(Box::from(word)),
                    _ => return Err(LexerErrorKind::UnknownToken)
                }
            }
        }))
    }

    fn add_token(&mut self, token_type: TokenType, start: Cursor) {
        self.tokens.push(Token {
            token_type,
            start: start.to_tuple(),
            end: self.cursor.to_tuple(),
        });
    }

    /// Consumes a single-line comment (aka skips to the end of the line and returns nothing)
    fn consume_single_line_comment(&mut self) -> LexerResult<Option<TokenType>> {
        self.eat_until(&['\n']);
        self.next();

        Ok(None)
    }

    /// Consumes a multi-line comment (skips until it reaches */)
    fn consume_multi_line_comment(&mut self) -> LexerResult<Option<TokenType>> {
        self.skip_until(&['*']);
        self.expect(&'*')?;
        if self.expect(&'/').is_err() {
            return self.consume_multi_line_comment();
        }

        Ok(None)
    }

    /// Attempts to return a [`TokenType::String`]
    fn consume_string(&mut self) -> LexerResult<TokenType> {
        let string = self.eat_until(&['"', '\n']).unwrap_or_default();
        self.expect(&'"')?;
        Ok(TokenType::String(Box::from(string)))
    }

    /// Attempts to return a [`TokenType::ShellCommand`]
    fn consume_shell_command(&mut self) -> LexerResult<TokenType> {
        let cmd_name = self
            .eat_until(&[' ', '\t', '\n', '('])
            .ok_or(LexerErrorKind::UnexpectedEnd)?;

        let cmd_args = match self.peek() {
            Some(' ' | '\t') => {
                self.next();
                self.eat_until(&['\n'])
            }
            Some('(') => {
                self.next();
                if let Some(res) = self.eat_until(&['\n', '\0', ')']) {
                    self.expect(&')')?;
                    Some(res)
                } else {
                    None
                }
            }
            _ => None,
        };

        Ok(TokenType::ShellCommand(Box::from((cmd_name, cmd_args))))
    }

    /// Returns true if the char is a valid character for an identifier, false otherwies
    fn is_valid_identifier(char: &char) -> bool {
        match char {
            '_' => true,
            _ => char.is_alphanumeric(),
        }
    }

    /// Attempts to parse and return an integer
    fn eat_number(&mut self, char: char) -> LexerResult<isize> {
        let mut collector = String::new();

        let mut count: u8 = 0;
        let mut error: Option<LexerErrorKind> = None;

        // We switch the mode depending on the first character:
        // if it begins with 0, it must be followed by a letter:
        //  b - binary
        //  o - octal
        //  d - decimal
        //  x - hexadecimal
        let radix = match char {
            '1'..='9' => {
                collector.push(char);
                count += 1;
                10
            }

            '0' => {
                let radix = match self.peek() {
                    Some('b') => 2,
                    Some('o') => 8,
                    Some('d') => 10,
                    Some('x') => 16,
                    _ => {
                        error = Some(LexerErrorKind::InvalidNumberNotation);
                        10
                    },
                };

                self.next();
                radix
            }

            _ => {
                return Err(LexerErrorKind::UnexpectedCharacter {
                    expected: "0..9".to_string(),
                    found: Some(char),
                })
            }
        };

        while count < self.max_int_len {
            let Some(char) = self.peek() else {
                break;
            };

            if char == &'_' {
                self.next();
                continue;
            }

            if !char.is_digit(radix) {
                break;
            }

            collector.push(*char);
            count += 1;
            self.next();
        }

        if let Some(err) = error {
            return Err(err);
        }

        use std::num::IntErrorKind;
        match isize::from_str_radix(&collector, radix) {
            Ok(num) => Ok(num),
            Err(err) => Err(match err.kind() {
                IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                    LexerErrorKind::IntegerOverflow(collector)
                }

                IntErrorKind::InvalidDigit => LexerErrorKind::UnexpectedCharacter {
                    expected: "0..9".to_string(),
                    found: None,
                },

                _ => LexerErrorKind::UnknownToken,
            }),
        }
    }

    /// Iterates until it reaches whitespace
    fn eat_word(&mut self) -> Option<String> {
        self.eat_until_conditional(|char| !Self::is_valid_identifier(char))
    }

    /// Iterates until it reaches the closing character
    fn eat_until(&mut self, term: &[char]) -> Option<String> {
        self.eat_until_conditional(|c| term.contains(c))
    }

    /// Iterates until it reaches the closing character
    fn eat_until_conditional<F>(&mut self, func: F) -> Option<String>
    where
        F: Fn(&char) -> bool,
    {
        let mut collector = String::new();

        while let Some(char) = self.peek() {
            if func(char) {
                break;
            }

            collector.push(*char);
            self.next();
        }

        if collector.is_empty() {
            None
        } else {
            Some(collector)
        }
    }

    fn skip_until(&mut self, term: &[char]) {
        while let Some(char) = self.peek() {
            if term.contains(char) {
                break;
            }

            self.next();
        }
    }

    /// Iterates to the next character
    fn next(&mut self) -> Option<char> {
        if let Some(char) = self.chars.next() {
            self.cursor.next(&char);
            Some(char)
        } else {
            None
        }
    }

    /// Iterates to the next character if the next character is equal to the char argument
    fn next_if_eq(&mut self, char: &char) -> Option<char> {
        if self.peek_is(char) {
            self.next()
        } else {
            None
        }
    }

    /// Expects a character to be there
    fn expect(&mut self, expected: &char) -> LexerResult<char> {
        let Some(char) = self.next_if_eq(expected) else {
            return Err(LexerErrorKind::UnexpectedCharacter {
                expected: expected.to_string(),
                found: None,
            });
        };

        if &char == expected {
            Ok(char)
        } else {
            Err(LexerErrorKind::UnexpectedCharacter {
                expected: expected.to_string(),
                found: Some(char),
            })
        }
    }

    /// Checks if the next character is equal to the char argument
    fn peek_is(&mut self, char: &char) -> bool {
        self.peek().eq(&Some(char))
    }

    /// Returns the next character if exists without iterating
    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}