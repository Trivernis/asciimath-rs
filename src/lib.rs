#![feature(test)]
extern crate test;

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate lazy_static;

use crate::elements::special::Expression;
use crate::parsing::tokenizer::Tokenizer;
use crate::parsing::tree_parser::TreeParser;

pub mod elements;
pub mod format;
pub mod parsing;
pub mod tokens;
pub(crate) mod utils;

/// Parses the contents of a string into an AsciiMath expression.
///
/// This function first uses a tokenizer to parse the input string into
/// a sequence of tokens. Then it uses those tokens with the TreeParser to create
/// an Expression tree that can then be converted into MathML.
///
/// Example:
///
/// ```rust
/// fn main() {
///     let expression = asciimath_rs::parse("sin(2x) + 3".to_string());
/// }
/// ```
pub fn parse(content: String) -> Expression {
    let mut tokenizer = Tokenizer::new(content);
    let tokens = tokenizer.parse();
    let mut tree_parser = TreeParser::new(tokens);

    tree_parser.parse()
}

#[cfg(test)]
mod tests {
    use crate::elements::group::{Brackets, Group, Matrix, Vector};
    use crate::elements::literal::{Literal, Number};
    use crate::elements::special::{Expression, Special, Sum};
    use crate::elements::Element;
    use crate::format::mathml::ToMathML;
    use crate::parse;
    use crate::parsing::tokenizer::Tokenizer;
    use crate::parsing::tree_parser::TreeParser;
    use crate::tokens::{Function, Grouping, Misc, Operation, Relation, Text, Token};
    use crate::utils::Boxed;
    use std::fs;
    use test::Bencher;

    #[test]
    fn it_tokenizes_expressions1() {
        let expression = "sum_(i=1)^n*sin(x)";
        let mut tokenizer = Tokenizer::new(expression.to_string());
        let tokens = tokenizer.parse();
        assert_eq!(
            tokens,
            vec![
                Token::Operation(Operation::Sum),
                Token::Misc(Misc::Sub),
                Token::Grouping(Grouping::RParen),
                Token::Text(Text::Symbol("i".to_string())),
                Token::Relation(Relation::Eq),
                Token::Text(Text::Number("1".to_string())),
                Token::Grouping(Grouping::LParen),
                Token::Misc(Misc::Pow),
                Token::Text(Text::Symbol("n".to_string())),
                Token::Operation(Operation::CDot),
                Token::Function(Function::Sin),
                Token::Grouping(Grouping::RParen),
                Token::Text(Text::Symbol("x".to_string())),
                Token::Grouping(Grouping::LParen),
            ]
        );
    }

    #[test]
    fn it_tokenizes_expressions2() {
        let expression = "G_(11) = 5.16e6 € * (215)/(170) = 6.53e6";
        let mut tokenizer = Tokenizer::new(expression.to_string());
        let tokens = tokenizer.parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text(Text::Symbol("G".to_string())),
                Token::Misc(Misc::Sub),
                Token::Grouping(Grouping::RParen),
                Token::Text(Text::Number("11".to_string())),
                Token::Grouping(Grouping::LParen),
                Token::Text(Text::Whitespace),
                Token::Relation(Relation::Eq),
                Token::Text(Text::Whitespace),
                Token::Text(Text::Number("5.16e6".to_string())),
                Token::Text(Text::Whitespace),
                Token::Text(Text::Symbol("€".to_string())),
                Token::Text(Text::Whitespace),
                Token::Operation(Operation::CDot),
                Token::Text(Text::Whitespace),
                Token::Grouping(Grouping::RParen),
                Token::Text(Text::Number("215".to_string())),
                Token::Grouping(Grouping::LParen),
                Token::Misc(Misc::AsciiFrac),
                Token::Grouping(Grouping::RParen),
                Token::Text(Text::Number("170".to_string())),
                Token::Grouping(Grouping::LParen),
                Token::Text(Text::Whitespace),
                Token::Relation(Relation::Eq),
                Token::Text(Text::Whitespace),
                Token::Text(Text::Number("6.53e6".to_string()))
            ]
        );
    }

    #[test]
    fn it_tokenizes_expressions3() {
        let expression = "[[1, 2],[3, 4]] // \\\n";
        let mut tokenizer = Tokenizer::new(expression.to_string());
        let tokens = tokenizer.parse();
        assert_eq!(
            tokens,
            vec![
                Token::Grouping(Grouping::RBracket),
                Token::Grouping(Grouping::RBracket),
                Token::Text(Text::Number("1".to_string())),
                Token::Grouping(Grouping::MSep),
                Token::Text(Text::Whitespace),
                Token::Text(Text::Number("2".to_string())),
                Token::Grouping(Grouping::LBracket),
                Token::Grouping(Grouping::MSep),
                Token::Grouping(Grouping::RBracket),
                Token::Text(Text::Number("3".to_string())),
                Token::Grouping(Grouping::MSep),
                Token::Text(Text::Whitespace),
                Token::Text(Text::Number("4".to_string())),
                Token::Grouping(Grouping::LBracket),
                Token::Grouping(Grouping::LBracket),
                Token::Text(Text::Whitespace),
                Token::Operation(Operation::Slash),
                Token::Text(Text::Whitespace),
                Token::Text(Text::NewLine),
            ]
        );
    }

    #[test]
    fn it_tokenizes_text1() {
        let expression = "\"just plain text\"";
        let mut tokenizer = Tokenizer::new(expression.to_string());
        let tokens = tokenizer.parse();
        assert_eq!(
            tokens,
            vec![Token::Text(Text::Plain("just plain text".to_string()))]
        )
    }

    #[test]
    fn it_tokenizes_text2() {
        let expression = "\"plain text\" * \"plain text 2\" + a";
        let mut tokenizer = Tokenizer::new(expression.to_string());
        let tokens = tokenizer.parse();
        assert_eq!(
            tokens,
            vec![
                Token::Text(Text::Plain("plain text".to_string())),
                Token::Text(Text::Whitespace),
                Token::Operation(Operation::CDot),
                Token::Text(Text::Whitespace),
                Token::Text(Text::Plain("plain text 2".to_string())),
                Token::Text(Text::Whitespace),
                Token::Operation(Operation::Plus),
                Token::Text(Text::Whitespace),
                Token::Text(Text::Symbol("a".to_string()))
            ]
        )
    }

    #[test]
    fn it_parses_into_a_tree1() {
        let expression = "sum_2^3";
        let mut tokenizer = Tokenizer::new(expression.to_string());
        let tokens = tokenizer.parse();
        let mut tree_parser = TreeParser::new(tokens.clone());
        let expression = tree_parser.parse();
        let mut test_expression = Expression::new();
        test_expression.add_child(Element::Special(Special::Sum(Sum {
            bottom: Some(
                Element::Literal(Literal::Number(Number {
                    number: "2".to_string(),
                }))
                .boxed(),
            ),
            top: Some(
                Element::Literal(Literal::Number(Number {
                    number: "3".to_string(),
                }))
                .boxed(),
            ),
        })));
        assert_eq!(expression, test_expression)
    }

    #[test]
    fn it_parses_matrices() {
        assert_eq!(
            parse("[[1, 2],[3,4]]".to_string()),
            Expression {
                children: vec![Element::Group(Group::Matrix(Matrix {
                    inner: vec![
                        vec![
                            Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "1".to_string()
                                })),]
                            },
                            Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "2".to_string()
                                })),]
                            }
                        ],
                        vec![
                            Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "3".to_string()
                                })),]
                            },
                            Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "4".to_string()
                                })),]
                            }
                        ]
                    ]
                }))]
            }
        );
    }

    #[test]
    fn it_rejects_invalid_matrices() {
        assert_eq!(
            parse("[[1, 3, 4],[3,4]]".to_string()),
            Expression {
                children: vec![Element::Group(Group::Brackets(Brackets {
                    inner: Expression {
                        children: vec![
                            Element::Group(Group::Brackets(Brackets {
                                inner: Expression {
                                    children: vec![
                                        Element::Literal(Literal::Number(Number {
                                            number: "1".to_string()
                                        })),
                                        Element::Group(Group::MSep),
                                        Element::Literal(Literal::Number(Number {
                                            number: "3".to_string()
                                        })),
                                        Element::Group(Group::MSep),
                                        Element::Literal(Literal::Number(Number {
                                            number: "4".to_string()
                                        }))
                                    ]
                                }
                                .boxed()
                            })),
                            Element::Group(Group::MSep),
                            Element::Group(Group::Brackets(Brackets {
                                inner: Expression {
                                    children: vec![
                                        Element::Literal(Literal::Number(Number {
                                            number: "3".to_string()
                                        })),
                                        Element::Group(Group::MSep),
                                        Element::Literal(Literal::Number(Number {
                                            number: "4".to_string()
                                        }))
                                    ]
                                }
                                .boxed()
                            }))
                        ]
                    }
                    .boxed()
                }))]
            }
        );
        assert_eq!(
            parse("[[1]]".to_string()),
            Expression {
                children: vec![Element::Group(Group::Brackets(Brackets {
                    inner: Expression {
                        children: vec![Element::Group(Group::Brackets(Brackets {
                            inner: Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "1".to_string()
                                })),]
                            }
                            .boxed()
                        })),]
                    }
                    .boxed()
                }))]
            }
        );
    }

    #[test]
    fn it_parses_vectors() {
        assert_eq!(
            parse("((1), (2))".to_string()),
            Expression {
                children: vec![Element::Group(Group::Vector(Vector {
                    inner: vec![
                        vec![Expression {
                            children: vec![Element::Literal(Literal::Number(Number {
                                number: "1".to_string()
                            }))]
                        }],
                        vec![Expression {
                            children: vec![Element::Literal(Literal::Number(Number {
                                number: "2".to_string()
                            }))]
                        }]
                    ]
                }))]
            }
        );
        assert_eq!(
            parse("((1, 3), (2, 5))".to_string()),
            Expression {
                children: vec![Element::Group(Group::Vector(Vector {
                    inner: vec![
                        vec![
                            Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "1".to_string()
                                }))]
                            },
                            Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "3".to_string()
                                }))]
                            }
                        ],
                        vec![
                            Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "2".to_string()
                                }))]
                            },
                            Expression {
                                children: vec![Element::Literal(Literal::Number(Number {
                                    number: "5".to_string()
                                }))]
                            }
                        ]
                    ]
                }))]
            }
        )
    }

    #[allow(dead_code)]
    #[test]
    fn it_writes_mathml() {
        let str_expression =
            "alpha sqrt 1 in NN implies 2^4 + \\\n<=> sum_(k = 1)^3 - ((1),(2))[[2, 3 + 3],[4, 5]]  + alpha";
        let expression = parse(str_expression.to_string());
        fs::write(
            "test-files/test.html",
            format!(
                "<html><body><pre>{}</pre><math>{}</math><pre>{:#?}</pre></body></html>",
                str_expression,
                expression.to_mathml(),
                expression,
            ),
        )
        .unwrap();
    }

    #[bench]
    fn bench_tokenizer(b: &mut Bencher) {
        let expression = "sqrt 1 in NN implies 2^4 + sum_(k = 1)^3 - ((1),(2)) [[2, 3 + 3],[4, 5]] (((((((nesting)))))))";
        b.iter(|| parse(expression.to_string()));
    }
}
