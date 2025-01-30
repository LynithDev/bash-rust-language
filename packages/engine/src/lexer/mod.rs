use std::{iter::Peekable, str::Chars};

use error::LexerResult;
use tokens::{LexerToken, LexerTokenKind, LexerTokenList};

use crate::{
    component::{ComponentErrors, ComponentIter},
    constants::{MAX_I32_LEN, MAX_I64_LEN},
    error::{EngineError, ErrorList},
    Cursor,
};

pub use error::{LexerError, LexerErrorKind};

mod error;
pub mod tokens;

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    errors: ErrorList,
    cursor: Cursor,
    tokens: LexerTokenList,
    max_int_len: u8,

    #[cfg(feature = "cli")]
    source: crate::error::SourceFile,
}

impl<'a> Lexer<'a> {
    pub fn create(
        input: &'a str,
        #[cfg(feature = "cli")] path: Option<std::path::PathBuf>,
    ) -> Self {
        Self::create_bits(
            input,
            #[cfg(feature = "cli")]
            path,
            MAX_I64_LEN,
        )
    }

    pub fn create_32b(
        input: &'a str,
        #[cfg(feature = "cli")] path: Option<std::path::PathBuf>,
    ) -> Self {
        Self::create_bits(
            input,
            #[cfg(feature = "cli")]
            path,
            MAX_I32_LEN,
        )
    }

    pub fn create_bits(
        input: &'a str,
        #[cfg(feature = "cli")] path: Option<std::path::PathBuf>,
        max_int_len: u8,
    ) -> Self {
        let lexer = Self {
            chars: input.trim().chars().peekable(),
            errors: ErrorList::new(),
            cursor: Cursor::create(),
            tokens: LexerTokenList::new(),
            max_int_len,

            #[cfg(feature = "cli")]
            source: Box::from((path, input.to_string())),
        };

        debug!("created lexer");
        lexer
    }

    pub fn tokenize(&mut self) -> &LexerTokenList {
        while self.peek().is_some() {
            let start = self.cursor;

            if let Some(char) = self.next() {
                match self.scan_char(&char) {
                    Ok(Some(token_type)) => self.add_token(token_type, start),
                    Err(err) => {
                        self.errors.push(EngineError::LexerError(LexerError {
                            #[cfg(feature = "cli")]
                            source_file: self.get_source_sliced(start, self.cursor),
                            start,
                            end: self.cursor,
                            kind: err,
                        }));
                    }
                    _ => {}
                }
            }
        }

        let start = self.cursor;
        self.cursor.next_line();
        self.add_token(LexerTokenKind::EOL, start);

        &self.tokens
    }

    fn scan_char(&mut self, char: &char) -> LexerResult<Option<LexerTokenKind>> {
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
                char == &$a && self.next_if_eq(&$b).is_some()
            };
        }

        use LexerTokenKind::*;

        Ok(Some(match char {
            ' ' => return Ok(None),
            '\n' => EOL,

            '=' => match self.peek() {
                Some(&'=') => {
                    self.next();
                    EqualEqual
                }

                Some(&'>') => {
                    self.next();
                    Arrow
                }

                _ => Equal,
            },
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
            _ if double!('&', '&') => And,
            _ if double!('|', '|') => Or,

            '(' => LParam,
            ')' => RParam,
            '{' => LBracket,
            '}' => RBracket,
            ',' => Comma,
            ':' => Colon,
            _ if double!('.', '.') => {
                if self.next() == Some('=') {
                    RangeInclusive
                } else {
                    Range
                }
            }

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
                    "while" => While,
                    "loop" => Loop,
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
                    _ => return Err(LexerErrorKind::UnknownToken),
                }
            }
        }))
    }

    fn add_token(&mut self, token_type: LexerTokenKind, start: Cursor) {
        self.tokens.push(LexerToken {
            kind: token_type,
            start,
            end: self.cursor,
        });
    }

    /// Consumes a single-line comment (aka skips to the end of the line and returns nothing)
    fn consume_single_line_comment(&mut self) -> LexerResult<Option<LexerTokenKind>> {
        self.eat_until(&['\n'], false);
        self.next();

        Ok(None)
    }

    /// Consumes a multi-line comment (skips until it reaches */)
    fn consume_multi_line_comment(&mut self) -> LexerResult<Option<LexerTokenKind>> {
        self.skip_until(&['*']);
        self.expect_char(&'*')?;
        if self.expect(&'/').is_err() {
            return self.consume_multi_line_comment();
        }

        Ok(None)
    }

    /// Attempts to return a [`TokenType::String`]
    fn consume_string(&mut self) -> LexerResult<LexerTokenKind> {
        let string = self.eat_until(&['"', '\n'], true).unwrap_or_default();
        self.expect_char(&'"')?;
        Ok(LexerTokenKind::String(Box::from(string)))
    }

    /// Attempts to return a [`TokenType::ShellCommand`]
    fn consume_shell_command(&mut self) -> LexerResult<LexerTokenKind> {
        let cmd_name = self
            .eat_until(&[' ', '\t', '\n', '('], false)
            .ok_or(LexerErrorKind::UnexpectedEnd)?;

        let cmd_args = match self.peek() {
            Some(' ' | '\t') => {
                self.next();
                self.eat_until(&['\n'], false)
            }
            Some('(') => {
                self.next();
                if let Some(res) = self.eat_until(&['\n', '\0', ')'], true) {
                    self.expect_char(&')')?;
                    Some(res)
                } else {
                    None
                }
            }
            _ => None,
        };

        Ok(LexerTokenKind::ShellCommand(Box::from((
            cmd_name, cmd_args,
        ))))
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
                    Some(char) if char.is_ascii_alphabetic() => match char {
                        'b' => 2,
                        'o' => 8,
                        'd' => 10,
                        'x' => 16,
                        _ => {
                            error = Some(LexerErrorKind::InvalidNumberNotation);
                            10
                        }
                    },
                    _ => return Ok(0),
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
        self.eat_until_conditional(|char| !Self::is_valid_identifier(char), false)
    }

    /// Iterates until it reaches the closing character
    fn eat_until(&mut self, term: &[char], escapeable: bool) -> Option<String> {
        self.eat_until_conditional(|c| term.contains(c), escapeable)
    }

    /// Iterates until it reaches the closing character
    fn eat_until_conditional<F>(&mut self, func: F, escapeable: bool) -> Option<String>
    where
        F: Fn(&char) -> bool,
    {
        let mut collector = String::new();

        while let Some(char) = self.peek() {
            if escapeable && char == &'\\' {
                self.next(); // Moves onto the \ char

                if let Some(char) = self.peek() {
                    let char = match char {
                        '0' => '\0',
                        't' => '\t',
                        'n' => '\n',
                        'r' => '\r',
                        _ => *char,
                    };

                    collector.push(char);
                    self.next();
                }

                continue;
            }

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

    fn expect_char(&mut self, expected: &char) -> LexerResult<char> {
        self.expect(expected)
            .map_err(|found| LexerErrorKind::UnexpectedCharacter {
                expected: expected.to_string(),
                found,
            })
    }
}

impl ComponentErrors for Lexer<'_> {
    fn fetch_errors(&self) -> &ErrorList {
        &self.errors
    }

    #[cfg(feature = "cli")]
    fn source(&self) -> &crate::error::SourceFile {
        &self.source
    }
}

impl<'a> ComponentIter<'a, char, char, Chars<'a>> for Lexer<'a> {
    fn get_iter(&mut self) -> &mut Peekable<Chars<'a>> {
        &mut self.chars
    }

    fn cursor_next(&mut self, item: &char) {
        self.cursor.next(item);
    }
}
