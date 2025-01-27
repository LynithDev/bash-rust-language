use engine_lib::{error::EngineResult, lexer::{Lexer, tokens::{Token, TokenType}}};
use pretty_assertions::assert_eq;

macro_rules! create_test {
    ($name:ident, $code:literal, [$($exp:expr),+]) => {
        #[test]
        fn $name() -> EngineResult<()> {
            let code = $code;

            let mut lexer = Lexer::create(code);
            let token_list = lexer.tokenize()?;

            let expected = vec![
                $($exp),+
            ];

            assert_eq!(&expected, token_list);
            Ok(())
        }
    };
}

create_test!(
    basic_variable, 
    "

var test = 50

    ", 
    [
        Token { start: (1, 1), end: (1, 4), token_type: TokenType::Var },
        Token { start: (1, 5), end: (1, 9), token_type: TokenType::Identifier(Box::from(String::from("test"))) },
        Token { start: (1, 10), end: (1, 11), token_type: TokenType::Equal },
        Token { start: (1, 12), end: (1, 14), token_type: TokenType::Integer(50) },
        Token { start: (1, 14), end: (2, 1), token_type: TokenType::EOL }
    ]
);

create_test!(
    multiline_basic_variable,
    "

var test = 50
var my_str = \"hello world\"

    ",
    [
        Token { start: (1, 1), end: (1, 4), token_type: TokenType::Var },
        Token { start: (1, 5), end: (1, 9), token_type: TokenType::Identifier(Box::from(String::from("test"))) },
        Token { start: (1, 10), end: (1, 11), token_type: TokenType::Equal },
        Token { start: (1, 12), end: (1, 14), token_type: TokenType::Integer(50) },
        Token { start: (1, 14), end: (2, 1), token_type: TokenType::EOL },

        Token { start: (2, 1), end: (2, 4), token_type: TokenType::Var },
        Token { start: (2, 5), end: (2, 11), token_type: TokenType::Identifier(Box::from(String::from("my_str"))) },
        Token { start: (2, 12), end: (2, 13), token_type: TokenType::Equal },
        Token { start: (2, 14), end: (2, 27), token_type: TokenType::String(Box::from(String::from("hello world"))) },
        Token { start: (2, 27), end: (3, 1), token_type: TokenType::EOL }
    ]
);

create_test!(
    multiline_shell_command_variable,
    "

var test1 = $echo hello world
var test2 = $echo(hello world)
var test3 = $echo(\"hello world\") + \"lol\"
var test4 = $echo hello world + \"lol\"

    ",
    [
        Token { start: (1, 1), end: (1, 4), token_type: TokenType::Var },
        Token { start: (1, 5), end: (1, 10), token_type: TokenType::Identifier(Box::from(String::from("test1"))) },
        Token { start: (1, 11), end: (1, 12), token_type: TokenType::Equal },
        Token { start: (1, 13), end: (1, 30), token_type: TokenType::ShellCommand(Box::from(String::from("echo")), Some(Box::from(String::from("hello world")))) },
        Token { start: (1, 30), end: (2, 1), token_type: TokenType::EOL },

        Token { start: (2, 1), end: (2, 4), token_type: TokenType::Var },
        Token { start: (2, 5), end: (2, 10), token_type: TokenType::Identifier(Box::from(String::from("test2"))) },
        Token { start: (2, 11), end: (2, 12), token_type: TokenType::Equal },
        Token { start: (2, 13), end: (2, 31), token_type: TokenType::ShellCommand(Box::from(String::from("echo")), Some(Box::from(String::from("hello world")))) },
        Token { start: (2, 31), end: (3, 1), token_type: TokenType::EOL },

        Token { start: (3, 1), end: (3, 4), token_type: TokenType::Var },
        Token { start: (3, 5), end: (3, 10), token_type: TokenType::Identifier(Box::from(String::from("test3"))) },
        Token { start: (3, 11), end: (3, 12), token_type: TokenType::Equal },
        Token { start: (3, 13), end: (3, 33), token_type: TokenType::ShellCommand(Box::from(String::from("echo")), Some(Box::from(String::from("\"hello world\"")))) },
        Token { start: (3, 34), end: (3, 35), token_type: TokenType::Plus },
        Token { start: (3, 36), end: (3, 41), token_type: TokenType::String(Box::from(String::from("lol"))) },
        Token { start: (3, 41), end: (4, 1), token_type: TokenType::EOL },

        Token { start: (4, 1), end: (4, 4), token_type: TokenType::Var },
        Token { start: (4, 5), end: (4, 10), token_type: TokenType::Identifier(Box::from(String::from("test4"))) },
        Token { start: (4, 11), end: (4, 12), token_type: TokenType::Equal },
        Token { start: (4, 13), end: (4, 38), token_type: TokenType::ShellCommand(Box::from(String::from("echo")), Some(Box::from(String::from("hello world + \"lol\"")))) },
        Token { start: (4, 38), end: (5, 1), token_type: TokenType::EOL }
    ]
);