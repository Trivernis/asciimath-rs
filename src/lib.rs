#![feature(test)]
extern crate test;

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate lazy_static;

pub mod elements;
pub mod parsing;
mod tokens;
mod utils;

#[cfg(test)]
mod tests {
    use crate::elements::literal::{Literal, Number};
    use crate::elements::special::{Expression, Special, Sum};
    use crate::elements::Element;
    use crate::parsing::tokenizer::Tokenizer;
    use crate::parsing::tree_parser::TreeParser;
    use crate::tokens::Function::Exp;
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
    fn it_parses_into_a_tree2() {
        let str_expression = "bb\"test\" bb";
        let mut tokenizer = Tokenizer::new(str_expression.to_string());
        let tokens = tokenizer.parse();
        let mut tree_parser = TreeParser::new(tokens.clone());
        let expression = tree_parser.parse();
        fs::write(
            "test-files/test.txt",
            format!("{}\n\n{:?}\n\n{:#?}", str_expression, tokens, expression),
        );
    }

    #[bench]
    fn bench_tokenizer(b: &mut Bencher) {
        let expression = "sum_(iiiiiiiii=1)^n i^3=((n(n+1))/2)^2";
        b.iter(|| {
            let mut tokenizer = Tokenizer::new(expression.to_string());
            let _ = tokenizer.parse();
        });
    }
}
