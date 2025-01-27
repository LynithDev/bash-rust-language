use std::{iter::Peekable, str::Chars};

use tokens::{Token, TokenList, TokenType};

use crate::{
    constants::{MAX_I32_LEN, MAX_I64_LEN},
    error::{EngineError, EngineResult, ErrorList},
};

pub mod tokens;

pub(super) type Cursor = (u16, u16);

#[derive(thiserror::Error, Debug, Clone)]
pub enum LexerError {
    #[error("unexpected end of input")]
    UnexpectedEnd,
    #[error("invalid number notation (valid notations are b(inary), o(ctal), d(ecimal), x(hex)")]
    InvalidNumberNotation,
    #[error("expected character '{expected}' but found {found:?}")]
    UnexpectedCharacter { expected: char, found: Option<char> },
    #[error("unknown token(s) at {}:{} to {}:{}", start.0, start.1, end.0, end.1)]
    UnknownToken { start: Cursor, end: Cursor }
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    cursor: Cursor,
    tokens: TokenList,
    max_int_len: u8,
    errors: ErrorList,
}

impl<'a> Lexer<'a> {
    pub fn create(input: &'a str) -> Self {
        Self {
            chars: input.trim().chars().peekable(),
            cursor: Self::create_cursor(),
            tokens: TokenList::new(),
            max_int_len: MAX_I64_LEN,
            errors: ErrorList::new(),
        }
    }

    pub fn create_32b(input: &'a str) -> Self {
        Self {
            chars: input.trim().chars().peekable(),
            cursor: Self::create_cursor(),
            tokens: TokenList::new(),
            max_int_len: MAX_I32_LEN,
            errors: ErrorList::new(),
        }
    }

    pub fn tokenize(&mut self) -> &TokenList {
        while self.peek().is_some() {
            let start = self.cursor;

            if let Some(char) = self.next() {
                match self.scan_char(&start, &char) {
                    Ok(Some(token_type)) => self.add_token(token_type, start),
                    Err(err) => self.errors.push(err),
                    _ => {}
                }
            }
        }

        let start = self.cursor;
        self.next_cursor_line();
        self.add_token(TokenType::EOL, start);

        &self.tokens
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn fetch_errors(&self) -> &ErrorList {
        &self.errors
    }


    fn scan_char(&mut self, start: &Cursor, char: &char) -> EngineResult<Option<TokenType>> {
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
                },
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
                    _ => return Err(EngineError::LexerError(LexerError::UnknownToken { start: *start, end: self.cursor }))
                }
            }
        }))
    }

    fn add_token(&mut self, token_type: TokenType, start: Cursor) {
        self.tokens.push(Token {
            token_type,
            start,
            end: self.cursor,
        });
    }

    /// Consumes a single-line comment (aka skips to the end of the line and returns nothing)
    fn consume_single_line_comment(&mut self) -> EngineResult<Option<TokenType>> {
        self.eat_until(&['\n']);
        self.next();

        Ok(None)
    }

    /// Consumes a multi-line comment (skips until it reaches */)
    fn consume_multi_line_comment(&mut self) -> EngineResult<Option<TokenType>> {
        self.skip_until(&['*']);
        self.expect(&'*')?;
        if self.expect(&'/').is_err() {
            return self.consume_multi_line_comment();
        }

        Ok(None)
    }

    /// Attempts to return a [`TokenType::String`]
    fn consume_string(&mut self) -> EngineResult<TokenType> {
        let string = self.eat_until(&['"']).unwrap_or_default();
        self.expect(&'"')?;
        Ok(TokenType::String(Box::from(string)))
    }

    /// Attempts to return a [`TokenType::ShellCommand`]
    fn consume_shell_command(&mut self) -> EngineResult<TokenType> {
        let cmd_name = self.eat_until(&[' ', '\t', '\n', '('])
            .ok_or(EngineError::LexerError(LexerError::UnexpectedEnd))?;

        let cmd_args = match self.peek() {
            Some(' ' | '\t') => {
                self.next();
                self.eat_until(&['\n'])
            },
            Some('(') => {
                self.next();
                if let Some(res) = self.eat_until(&[')']) {
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
    fn eat_number(&mut self, char: char) -> EngineResult<isize> {
        let mut collector = String::new();

        let mut count: u8 = 0;

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
                    _ => return Err(EngineError::LexerError(LexerError::InvalidNumberNotation)),
                };

                self.next();
                radix
            }

            _ => return Err(EngineError::Unreachable),
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

        Ok(isize::from_str_radix(&collector, radix)?)
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
    where F: Fn(&char) -> bool {
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
            self.next_cursor(&char);
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
    fn expect(&mut self, expected: &char) -> EngineResult<char> {
        let Some(char) = self.next() else {
            return Err(EngineError::LexerError(LexerError::UnexpectedCharacter {
                expected: *expected,
                found: None,
            }))
        };

        if &char == expected {
            Ok(char)
        } else {
            Err(EngineError::LexerError(LexerError::UnexpectedCharacter {
                expected: *expected,
                found: Some(char),
            }))
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

    /// Goes to the new line if needed, based on the character
    fn next_cursor(&mut self, char: &char) {
        if char.eq(&'\n') {
            self.next_cursor_line();
        } else {
            self.next_cursor_char();
        }
    }

    /// Moves the cursor to the next column
    fn next_cursor_char(&mut self) {
        self.cursor.1 += 1;
    }

    /// Moves the cursor to the next line and resets the column to 0
    fn next_cursor_line(&mut self) {
        self.cursor = (self.cursor.0 + 1, 1);
    }

    #[inline]
    const fn create_cursor() -> Cursor {
        (1, 1)
    }
}