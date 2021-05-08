use crate::elements::group::{Brackets, Group, Matrix, Parentheses, Vector};
use crate::elements::literal::{Literal, Number};
use crate::elements::special::{Expression, Root, Special, Sum};
use crate::elements::Element;
use crate::parse;
use crate::parsing::tokenizer::Tokenizer;
use crate::parsing::tree_parser::TreeParser;
use crate::tokens::{Function, Operation};
use crate::utils::Boxed;

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
        parse("((1), (2))(1,2) - f".to_string()),
        Expression {
            children: vec![
                Element::Group(Group::Vector(Vector {
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
                })),
                Element::Group(Group::Parentheses(Parentheses {
                    inner: Expression {
                        children: vec![
                            Element::Literal(Literal::Number(Number {
                                number: "1".to_string()
                            })),
                            Element::Group(Group::MSep),
                            Element::Literal(Literal::Number(Number {
                                number: "2".to_string()
                            }))
                        ]
                    }
                    .boxed()
                })),
                Element::Literal(Literal::Operation(Operation::Minus)),
                Element::Literal(Literal::Function(Function::F))
            ]
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

#[test]
fn it_parses_roots() {
    let expr = parse("root 3 16".to_string());
    assert_eq!(
        expr,
        Expression {
            children: vec![Element::Special(Special::Root(Root {
                base: Element::Literal(Literal::Number(Number {
                    number: "3".to_string()
                }))
                .boxed(),
                inner: Element::Literal(Literal::Number(Number {
                    number: "16".to_string()
                }))
                .boxed(),
            }))]
        }
    )
}
