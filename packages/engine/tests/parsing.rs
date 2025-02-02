macro_rules! token_list_comparison {
    ($name:ident, $code:tt, [$($exp:expr),+]) => {
        #[test]
        fn $name() -> lang_engine::error::EngineResult<()> {
            #[allow(unused_imports)]
            use lang_engine::{parser::ast::{Statement, Expression, Variable, Function, Literal}, cursor::{WithCursor, Cursor}, component::ComponentErrors};
            let code = $code;

            // Step 1
            let mut lexer = lang_engine::lexer::Lexer::create(code, None);
            lexer.tokens();

            if lexer.has_errors() {
                println!("{:#?}", lexer.fetch_errors());
            }

            // Step 2
            let source = lexer.source().clone();
            let mut parser = lang_engine::parser::Parser::create(lexer.tokens(), source);
            parser.parse();

            let expected = vec![
                $($exp),+
            ];

            pretty_assertions::assert_eq!(&expected, parser.parse());

            if parser.has_errors() {
                println!("{:#?}", parser.fetch_errors());
            }

            Ok(())
        }
    };
}

token_list_comparison!(
    basic_variable,
    "var test = 50",
    [Statement::Variable(Box::new(Variable {
        name: String::from("test"),
        strict_type: None,
        value: Some(WithCursor {
            value: Expression::Literal(Box::new(Literal::Integer(50))),
            start: Cursor::from(1, 12),
            end: Cursor::from(1, 14),
        })
    }))]
);

token_list_comparison!(
    for_loop,
    "
for i in 0..5 {
    $echo #{i}
}
    ",
    [Statement::For(Box::from((
        Variable {
            name: String::from("i"),
            strict_type: None,
            value: None,
        },
        WithCursor::create_with(
            Cursor::from_full(1, 10, 9),
            Cursor::from_full(1, 14, 13),
            Expression::Range(Box::from((
                WithCursor::create_with(
                    Cursor::from_full(1, 10, 9),
                    Cursor::from_full(1, 11, 10),
                    Expression::Literal(Box::from(Literal::Integer(0)))
                ),
                WithCursor::create_with(
                    Cursor::from_full(1, 13, 12),
                    Cursor::from_full(1, 14, 13),
                    Expression::Literal(Box::from(Literal::Integer(5)))
                ),
                false
            )))
        ),
        WithCursor::create_with(
            Cursor::from_full(2, 1, 16),
            Cursor::from_full(3, 2, 32),
            vec![
                Statement::Expression(Box::from(
                    WithCursor::create_with(
                        Cursor::from_full(2, 5, 20),
                        Cursor::from_full(2, 15, 30),
                        Expression::ShellCommand(
                            Box::from((
                                String::from("echo"),
                                Some(
                                    String::from("#{i}"),
                                ),
                            )),
                        )),
                    ),
                ),
            ],
        ),
    )))]
);
