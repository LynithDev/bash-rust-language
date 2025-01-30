// macro_rules! token_list_comparison {
//     ($name:ident, $code:literal, [$($exp:expr),+]) => {
//         #[test]
//         fn $name() -> lang_engine::error::EngineResult<()> {
//             let code = $code;

//             let mut lexer = lang_engine::lexer::Lexer::create(code, None);
//             let token_list = lexer.tokenize();

//             let expected = vec![
//                 $($exp),+
//             ];

//             pretty_assertions::assert_eq!(&expected, token_list);

//             if lexer.has_errors() {
//                 println!("{:#?}", lexer.fetch_errors());
//             }

//             Ok(())
//         }
//     };
// }

// macro_rules! custom_assert {
//     ($name:ident, $code:literal, ($lexer:ident) => $block:block) => {
//         #[test]
//         fn $name() -> lang_engine::error::EngineResult<()> {
//             let code = $code;

//             let mut $lexer = lang_engine::lexer::Lexer::create(code, None);
//             $lexer.tokenize();

//             $block
//         }
//     };
// }

// mod basic_syntax {
//     use lang_engine::lexer::tokens::{LexerToken, LexerTokenKind};

//     token_list_comparison!(
//         single_line_comment,
//         "// hello world",
//         [
//             LexerToken { start: (1, 15), end: (2, 1), kind: LexerTokenKind::EOL }
//         ]
//     );

//     token_list_comparison!(
//         multi_line_comment,
//         "

// /* 
//     this demonstrates the 
//     multi line comment feature
//     of the language !!
// */ var test = 50

//         ",
//         [
//             LexerToken {
//                 start: (5, 4),
//                 end: (5, 7),
//                 kind: LexerTokenKind::Var
//             },
//             LexerToken {
//                 start: (5, 8),
//                 end: (5, 12),
//                 kind: LexerTokenKind::Identifier(Box::from(String::from("test")))
//             },
//             LexerToken {
//                 start: (5, 13),
//                 end: (5, 14),
//                 kind: LexerTokenKind::Equal
//             },
//             LexerToken {
//                 start: (5, 15),
//                 end: (5, 17),
//                 kind: LexerTokenKind::Integer(50)
//             },
//             LexerToken {
//                 start: (5, 17),
//                 end: (6, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         basic_variable,
//         "var test = 50",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 4),
//                 kind: LexerTokenKind::Var
//             },
//             LexerToken {
//                 start: (1, 5),
//                 end: (1, 9),
//                 kind: LexerTokenKind::Identifier(Box::from(String::from("test")))
//             },
//             LexerToken {
//                 start: (1, 10),
//                 end: (1, 11),
//                 kind: LexerTokenKind::Equal
//             },
//             LexerToken {
//                 start: (1, 12),
//                 end: (1, 14),
//                 kind: LexerTokenKind::Integer(50)
//             },
//             LexerToken {
//                 start: (1, 14),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         multiline_basic_variable,
//         "

// var test = 50
// var my_str = \"hello world\"

//     ",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 4),
//                 kind: LexerTokenKind::Var
//             },
//             LexerToken {
//                 start: (1, 5),
//                 end: (1, 9),
//                 kind: LexerTokenKind::Identifier(Box::from(String::from("test")))
//             },
//             LexerToken {
//                 start: (1, 10),
//                 end: (1, 11),
//                 kind: LexerTokenKind::Equal
//             },
//             LexerToken {
//                 start: (1, 12),
//                 end: (1, 14),
//                 kind: LexerTokenKind::Integer(50)
//             },
//             LexerToken {
//                 start: (1, 14),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             },
//             LexerToken {
//                 start: (2, 1),
//                 end: (2, 4),
//                 kind: LexerTokenKind::Var
//             },
//             LexerToken {
//                 start: (2, 5),
//                 end: (2, 11),
//                 kind: LexerTokenKind::Identifier(Box::from(String::from("my_str")))
//             },
//             LexerToken {
//                 start: (2, 12),
//                 end: (2, 13),
//                 kind: LexerTokenKind::Equal
//             },
//             LexerToken {
//                 start: (2, 14),
//                 end: (2, 27),
//                 kind: LexerTokenKind::String(Box::from(String::from("hello world")))
//             },
//             LexerToken {
//                 start: (2, 27),
//                 end: (3, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         multiline_shell_command_variable,
//         "

// var test1 = $echo hello world
// var test2 = $echo(hello world)
// var test3 = $echo(\"hello world\") + \"lol\"
// var test4 = $echo hello world + \"lol\"

//     ",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 4),
//                 kind: LexerTokenKind::Var
//             },
//             LexerToken {
//                 start: (1, 5),
//                 end: (1, 10),
//                 kind: LexerTokenKind::Identifier(Box::from(String::from("test1")))
//             },
//             LexerToken {
//                 start: (1, 11),
//                 end: (1, 12),
//                 kind: LexerTokenKind::Equal
//             },
//             LexerToken {
//                 start: (1, 13),
//                 end: (1, 30),
//                 kind: LexerTokenKind::ShellCommand(Box::from((
//                     String::from("echo"),
//                     Some(String::from("hello world"))
//                 )))
//             },
//             LexerToken {
//                 start: (1, 30),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             },
//             LexerToken {
//                 start: (2, 1),
//                 end: (2, 4),
//                 kind: LexerTokenKind::Var
//             },
//             LexerToken {
//                 start: (2, 5),
//                 end: (2, 10),
//                 kind: LexerTokenKind::Identifier(Box::from(String::from("test2")))
//             },
//             LexerToken {
//                 start: (2, 11),
//                 end: (2, 12),
//                 kind: LexerTokenKind::Equal
//             },
//             LexerToken {
//                 start: (2, 13),
//                 end: (2, 31),
//                 kind: LexerTokenKind::ShellCommand(Box::from((
//                     String::from("echo"),
//                     Some(String::from("hello world"))
//                 )))
//             },
//             LexerToken {
//                 start: (2, 31),
//                 end: (3, 1),
//                 kind: LexerTokenKind::EOL
//             },
//             LexerToken {
//                 start: (3, 1),
//                 end: (3, 4),
//                 kind: LexerTokenKind::Var
//             },
//             LexerToken {
//                 start: (3, 5),
//                 end: (3, 10),
//                 kind: LexerTokenKind::Identifier(Box::from(String::from("test3")))
//             },
//             LexerToken {
//                 start: (3, 11),
//                 end: (3, 12),
//                 kind: LexerTokenKind::Equal
//             },
//             LexerToken {
//                 start: (3, 13),
//                 end: (3, 33),
//                 kind: LexerTokenKind::ShellCommand(Box::from((
//                     String::from("echo"),
//                     Some(String::from("\"hello world\""))
//                 )))
//             },
//             LexerToken {
//                 start: (3, 34),
//                 end: (3, 35),
//                 kind: LexerTokenKind::Plus
//             },
//             LexerToken {
//                 start: (3, 36),
//                 end: (3, 41),
//                 kind: LexerTokenKind::String(Box::from(String::from("lol")))
//             },
//             LexerToken {
//                 start: (3, 41),
//                 end: (4, 1),
//                 kind: LexerTokenKind::EOL
//             },
//             LexerToken {
//                 start: (4, 1),
//                 end: (4, 4),
//                 kind: LexerTokenKind::Var
//             },
//             LexerToken {
//                 start: (4, 5),
//                 end: (4, 10),
//                 kind: LexerTokenKind::Identifier(Box::from(String::from("test4")))
//             },
//             LexerToken {
//                 start: (4, 11),
//                 end: (4, 12),
//                 kind: LexerTokenKind::Equal
//             },
//             LexerToken {
//                 start: (4, 13),
//                 end: (4, 38),
//                 kind: LexerTokenKind::ShellCommand(Box::from((
//                     String::from("echo"),
//                     Some(String::from("hello world + \"lol\""))
//                 )))
//             },
//             LexerToken {
//                 start: (4, 38),
//                 end: (5, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );
// }

// mod integer_parsing {
//     use lang_engine::lexer::tokens::{LexerToken, LexerTokenKind};

//     token_list_comparison!(
//         integer_parsing,
//         "51",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 3),
//                 kind: LexerTokenKind::Integer(51)
//             },
//             LexerToken {
//                 start: (1, 3),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         negative_integer_parsing,
//         "-51",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 4),
//                 kind: LexerTokenKind::Integer(-51)
//             },
//             LexerToken {
//                 start: (1, 4),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         binary_integer_parsing,
//         "0b110_110",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 10),
//                 kind: LexerTokenKind::Integer(54)
//             },
//             LexerToken {
//                 start: (1, 10),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         hex_integer_parsing,
//         "0xff",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 5),
//                 kind: LexerTokenKind::Integer(255)
//             },
//             LexerToken {
//                 start: (1, 5),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         negative_hex_integer_parsing,
//         "-0xff",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 6),
//                 kind: LexerTokenKind::Integer(-255)
//             },
//             LexerToken {
//                 start: (1, 6),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         octal_integer_parsing,
//         "0o14",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 5),
//                 kind: LexerTokenKind::Integer(12)
//             },
//             LexerToken {
//                 start: (1, 5),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         decimal_integer_parsing,
//         "0d12",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 5),
//                 kind: LexerTokenKind::Integer(12)
//             },
//             LexerToken {
//                 start: (1, 5),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     token_list_comparison!(
//         integer_64bit_max,
//         "
// 9_223_372_036_854_775_807
// -9_223_372_036_854_775_807
//     ",
//         [
//             LexerToken {
//                 start: (1, 1),
//                 end: (1, 26),
//                 kind: LexerTokenKind::Integer(9_223_372_036_854_775_807)
//             },
//             LexerToken {
//                 start: (1, 26),
//                 end: (2, 1),
//                 kind: LexerTokenKind::EOL
//             },
//             LexerToken {
//                 start: (2, 1),
//                 end: (2, 27),
//                 kind: LexerTokenKind::Integer(-9_223_372_036_854_775_807)
//             },
//             LexerToken {
//                 start: (2, 27),
//                 end: (3, 1),
//                 kind: LexerTokenKind::EOL
//             }
//         ]
//     );

//     custom_assert!(
//         integer_overflow,
//         "9_999_999_999_999_999_999",
//         (lexer) => {
//             assert!(lexer.has_errors());
//             if let Some(err) = lexer.fetch_errors().first() {
//                 match err {
//                     lang_engine::error::EngineError::LexerError(err) => {
//                         pretty_assertions::assert_eq!(
//                             err.kind, 
//                             lang_engine::lexer::LexerErrorKind::IntegerOverflow("9999999999999999999".to_string())
//                         );
//                     },
//                     err => return Err(err.clone())
//                 }
//             }

//             Ok(())
//         }
//     );
// }
