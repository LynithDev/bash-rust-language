use engine_lib::{error::EngineResult, lexer::{self, tokens::{Token, TokenType}}};
use pretty_assertions::assert_eq;

macro_rules! create_test {
    ($name:ident, $code:literal, [$($exp:expr),+]) => {
        #[test]
        fn $name() -> EngineResult<()> {
            let code = String::from($code);

            let token_list = lexer::tokenize(code)?;
            let expected = vec![
                $($exp),+
            ];

            assert_eq!(token_list, expected);
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
        Token { col: 1, line: 1, token_type: TokenType::Var },
        Token { col: 5, line: 1, token_type: TokenType::Identifier(String::from("test")) },
        Token { col: 10, line: 1, token_type: TokenType::Equal },
        Token { col: 12, line: 1, token_type: TokenType::Integer(50) },
        Token { col: 14, line: 1, token_type: TokenType::EOL }
    ]
);

create_test!(
    multiline_basic_variable,
    "

var test = 50
var my_str = \"hello world\"

    ",
    [
        Token { col: 1, line: 1, token_type: TokenType::Var },
        Token { col: 5, line: 1, token_type: TokenType::Identifier(String::from("test")) },
        Token { col: 10, line: 1, token_type: TokenType::Equal },
        Token { col: 12, line: 1, token_type: TokenType::Integer(50) },
        Token { col: 14, line: 1, token_type: TokenType::EOL },

        Token { col: 1, line: 2, token_type: TokenType::Var },
        Token { col: 5, line: 2, token_type: TokenType::Identifier(String::from("my_str")) },
        Token { col: 12, line: 2, token_type: TokenType::Equal },
        Token { col: 14, line: 2, token_type: TokenType::String(String::from("hello world")) },
        Token { col: 27, line: 2, token_type: TokenType::EOL }
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
        Token { col: 1, line: 1, token_type: TokenType::Var },
        Token { col: 5, line: 1, token_type: TokenType::Identifier(String::from("test1")) },
        Token { col: 11, line: 1, token_type: TokenType::Equal },
        Token { col: 13, line: 1, token_type: TokenType::ShellCommand(String::from("echo"), Some(String::from("hello world"))) },
        Token { col: 30, line: 1, token_type: TokenType::EOL },

        Token { col: 1, line: 2, token_type: TokenType::Var },
        Token { col: 5, line: 2, token_type: TokenType::Identifier(String::from("test2")) },
        Token { col: 11, line: 2, token_type: TokenType::Equal },
        Token { col: 13, line: 2, token_type: TokenType::ShellCommand(String::from("echo"), Some(String::from("hello world"))) },
        Token { col: 31, line: 2, token_type: TokenType::EOL },

        Token { col: 1, line: 3, token_type: TokenType::Var },
        Token { col: 5, line: 3, token_type: TokenType::Identifier(String::from("test3")) },
        Token { col: 11, line: 3, token_type: TokenType::Equal },
        Token { col: 13, line: 3, token_type: TokenType::ShellCommand(String::from("echo"), Some(String::from("\"hello world\""))) },
        Token { col: 34, line: 3, token_type: TokenType::Plus },
        Token { col: 36, line: 3, token_type: TokenType::String(String::from("lol")) },
        Token { col: 41, line: 3, token_type: TokenType::EOL },

        Token { col: 1, line: 4, token_type: TokenType::Var },
        Token { col: 5, line: 4, token_type: TokenType::Identifier(String::from("test4")) },
        Token { col: 11, line: 4, token_type: TokenType::Equal },
        Token { col: 13, line: 4, token_type: TokenType::ShellCommand(String::from("echo"), Some(String::from("hello world + \"lol\""))) },
        Token { col: 38, line: 4, token_type: TokenType::EOL }
    ]
);