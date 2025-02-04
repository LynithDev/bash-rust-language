macro_rules! token_list_comparison {
    ($name:ident, $code:literal, [$($exp:expr),+]) => {
        #[test]
        fn $name() -> lang_engine::error::EngineResult<()> {
            let code = $code;

            let source_file = lang_engine::error::SourceFile::from(code.to_string(), None);
            let mut lexer = lang_engine::lexer::Lexer::create(&source_file);

            let expected = vec![
                $($exp),+
            ];

            pretty_assertions::assert_eq!(&expected, lexer.tokens());

            use lang_engine::{cursor::Cursor, component::ComponentErrors};
            if lexer.has_errors() {
                println!("{:#?}", lexer.fetch_errors());
            }

            Ok(())
        }
    };
}

macro_rules! custom_assert {
    ($name:ident, $code:literal, ($lexer:ident) => $block:block) => {
        #[test]
        fn $name() -> lang_engine::error::EngineResult<()> {
            use lang_engine::component::ComponentErrors;
            let code = $code;

            let source_file = lang_engine::error::SourceFile::from(code.to_string(), None);
            let mut $lexer = lang_engine::lexer::Lexer::create(&source_file);
            $lexer.tokens();

            $block
        }
    };
}

mod basic_syntax {
    use lang_engine::lexer::tokens::{LexerLiteral, LexerToken, LexerTokenKind};

    token_list_comparison!(
        single_line_comment,
        "// hello world",
        [LexerToken {
            value: None,
            start: Cursor::from(1, 15),
            end: Cursor::from(1, 15),
            kind: LexerTokenKind::EOF
        }]
    );

    token_list_comparison!(
        multi_line_comment,
        "

/* 
    this demonstrates the 
    multi line comment feature
    of the language !!
*/ var test = 50

        ",
        [
            LexerToken {
                value: None,
                start: Cursor::from(5, 4),
                end: Cursor::from(5, 7),
                kind: LexerTokenKind::Var
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Identifier(Box::from(String::from("test"))))),
                start: Cursor::from(5, 8),
                end: Cursor::from(5, 12),
                kind: LexerTokenKind::Identifier
            },
            LexerToken {
                value: None,
                start: Cursor::from(5, 13),
                end: Cursor::from(5, 14),
                kind: LexerTokenKind::Equal
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(50))),
                start: Cursor::from(5, 15),
                end: Cursor::from(5, 17),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(5, 17),
                end: Cursor::from(5, 17),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        basic_variable,
        "var test = 50",
        [
            LexerToken {
                value: None,
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 4),
                kind: LexerTokenKind::Var
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Identifier(Box::from(String::from("test"))))),
                start: Cursor::from(1, 5),
                end: Cursor::from(1, 9),
                kind: LexerTokenKind::Identifier
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 10),
                end: Cursor::from(1, 11),
                kind: LexerTokenKind::Equal
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(50))),
                start: Cursor::from(1, 12),
                end: Cursor::from(1, 14),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 14),
                end: Cursor::from(1, 14),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        multiline_basic_variable,
        "

var test = 50
var my_str = \"hello world\"

    ",
        [
            LexerToken {
                value: None,
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 4),
                kind: LexerTokenKind::Var
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Identifier(Box::from(String::from("test"))))),
                start: Cursor::from(1, 5),
                end: Cursor::from(1, 9),
                kind: LexerTokenKind::Identifier
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 10),
                end: Cursor::from(1, 11),
                kind: LexerTokenKind::Equal
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(50))),
                start: Cursor::from(1, 12),
                end: Cursor::from(1, 14),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 14),
                end: Cursor::from(2, 1),
                kind: LexerTokenKind::EOL
            },
            LexerToken {
                value: None,
                start: Cursor::from(2, 1),
                end: Cursor::from(2, 4),
                kind: LexerTokenKind::Var
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Identifier(Box::from(String::from("my_str"))))),
                start: Cursor::from(2, 5),
                end: Cursor::from(2, 11),
                kind: LexerTokenKind::Identifier
            },
            LexerToken {
                value: None,
                start: Cursor::from(2, 12),
                end: Cursor::from(2, 13),
                kind: LexerTokenKind::Equal
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::String(Box::from(String::from("hello world"))))),
                start: Cursor::from(2, 14),
                end: Cursor::from(2, 27),
                kind: LexerTokenKind::String
            },
            LexerToken {
                value: None,
                start: Cursor::from(2, 27),
                end: Cursor::from(2, 27),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        multiline_shell_command_variable,
        "

var test1 = $echo hello world
var test2 = $echo(hello world)
var test3 = $echo(\"hello world\") + \"lol\"
var test4 = $echo hello world + \"lol\"

    ",
        [
            LexerToken {
                value: None,
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 4),
                kind: LexerTokenKind::Var
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Identifier(Box::from(String::from("test1"))))),
                start: Cursor::from(1, 5),
                end: Cursor::from(1, 10),
                kind: LexerTokenKind::Identifier
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 11),
                end: Cursor::from(1, 12),
                kind: LexerTokenKind::Equal
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::ShellCommand(Box::from((String::from("echo"), Some(String::from("hello world"))))))),
                start: Cursor::from(1, 13),
                end: Cursor::from(1, 30),
                kind: LexerTokenKind::ShellCommand
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 30),
                end: Cursor::from(2, 1),
                kind: LexerTokenKind::EOL
            },
            LexerToken {
                value: None,
                start: Cursor::from(2, 1),
                end: Cursor::from(2, 4),
                kind: LexerTokenKind::Var
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Identifier(Box::from(String::from("test2"))))),
                start: Cursor::from(2, 5),
                end: Cursor::from(2, 10),
                kind: LexerTokenKind::Identifier
            },
            LexerToken {
                value: None,
                start: Cursor::from(2, 11),
                end: Cursor::from(2, 12),
                kind: LexerTokenKind::Equal
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::ShellCommand(Box::from((String::from("echo"), Some(String::from("hello world"))))))),
                start: Cursor::from(2, 13),
                end: Cursor::from(2, 31),
                kind: LexerTokenKind::ShellCommand
            },
            LexerToken {
                value: None,
                start: Cursor::from(2, 31),
                end: Cursor::from(3, 1),
                kind: LexerTokenKind::EOL
            },
            LexerToken {
                value: None,
                start: Cursor::from(3, 1),
                end: Cursor::from(3, 4),
                kind: LexerTokenKind::Var
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Identifier(Box::from(String::from("test3"))))),
                start: Cursor::from(3, 5),
                end: Cursor::from(3, 10),
                kind: LexerTokenKind::Identifier
            },
            LexerToken {
                value: None,
                start: Cursor::from(3, 11),
                end: Cursor::from(3, 12),
                kind: LexerTokenKind::Equal
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::ShellCommand(Box::from((String::from("echo"), Some(String::from("\"hello world\""))))))),
                start: Cursor::from(3, 13),
                end: Cursor::from(3, 33),
                kind: LexerTokenKind::ShellCommand
            },
            LexerToken {
                value: None,
                start: Cursor::from(3, 34),
                end: Cursor::from(3, 35),
                kind: LexerTokenKind::Plus
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::String(Box::from(String::from("lol"))))),
                start: Cursor::from(3, 36),
                end: Cursor::from(3, 41),
                kind: LexerTokenKind::String
            },
            LexerToken {
                value: None,
                start: Cursor::from(3, 41),
                end: Cursor::from(4, 1),
                kind: LexerTokenKind::EOL
            },
            LexerToken {
                value: None,
                start: Cursor::from(4, 1),
                end: Cursor::from(4, 4),
                kind: LexerTokenKind::Var
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Identifier(Box::from(String::from("test4"))))),
                start: Cursor::from(4, 5),
                end: Cursor::from(4, 10),
                kind: LexerTokenKind::Identifier
            },
            LexerToken {
                value: None,
                start: Cursor::from(4, 11),
                end: Cursor::from(4, 12),
                kind: LexerTokenKind::Equal
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::ShellCommand(Box::from((String::from("echo"), Some(String::from("hello world + \"lol\""))))))),
                start: Cursor::from(4, 13),
                end: Cursor::from(4, 38),
                kind: LexerTokenKind::ShellCommand
            },
            LexerToken {
                value: None,
                start: Cursor::from(4, 38),
                end: Cursor::from(4, 38),
                kind: LexerTokenKind::EOF
            }
        ]
    );
}

mod integer_parsing {
    use lang_engine::lexer::tokens::{LexerLiteral, LexerToken, LexerTokenKind};

    token_list_comparison!(
        integer_parsing,
        "51",
        [
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(51))),
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 3),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 3),
                end: Cursor::from(1, 3),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        negative_integer_parsing,
        "-51",
        [
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(-51))),
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 4),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 4),
                end: Cursor::from(1, 4),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        binary_integer_parsing,
        "0b110_110",
        [
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(54))),
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 10),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 10),
                end: Cursor::from(1, 10),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        hex_integer_parsing,
        "0xff",
        [
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(255))),
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 5),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 5),
                end: Cursor::from(1, 5),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        negative_hex_integer_parsing,
        "-0xff",
        [
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(-255))),
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 6),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 6),
                end: Cursor::from(1, 6),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        octal_integer_parsing,
        "0o14",
        [
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(12))),
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 5),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 5),
                end: Cursor::from(1, 5),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        decimal_integer_parsing,
        "0d12",
        [
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(12))),
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 5),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 5),
                end: Cursor::from(1, 5),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    token_list_comparison!(
        integer_64bit_max,
        "
9_223_372_036_854_775_807
-9_223_372_036_854_775_807
    ",
        [
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(9_223_372_036_854_775_807))),
                start: Cursor::from(1, 1),
                end: Cursor::from(1, 26),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(1, 26),
                end: Cursor::from(2, 1),
                kind: LexerTokenKind::EOL
            },
            LexerToken {
                value: Some(Box::from(LexerLiteral::Integer(-9_223_372_036_854_775_807))),
                start: Cursor::from(2, 1),
                end: Cursor::from(2, 27),
                kind: LexerTokenKind::Integer
            },
            LexerToken {
                value: None,
                start: Cursor::from(2, 27),
                end: Cursor::from(2, 27),
                kind: LexerTokenKind::EOF
            }
        ]
    );

    custom_assert!(
        integer_overflow,
        "9_999_999_999_999_999_999",
        (lexer) => {
            assert!(lexer.has_errors());
            if let Some(err) = lexer.fetch_errors().first() {
                pretty_assertions::assert_eq!(
                    err.kind,
                    lang_engine::lexer::LexerErrorKind::IntegerOverflow("9999999999999999999".to_string())
                );
            }

            Ok(())
        }
    );
}
