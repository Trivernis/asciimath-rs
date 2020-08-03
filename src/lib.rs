#![feature(test)]
extern crate test;

#[macro_use]
extern crate maplit;

#[macro_use]
extern crate lazy_static;

pub mod elements;
pub mod tokenizer;
mod tokens;

#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;
    use crate::tokens::{Grouping, Misc, Operation, Relation, Text, Token};
    use test::Bencher;

    #[test]
    fn it_tokenizes_expressions1() {
        let expression = "sum_(i=1)^n";
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
                Token::Text(Text::Symbol("n".to_string()))
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

    #[bench]
    fn bench_tokenizer(b: &mut Bencher) {
        let expression = "sum_(iiiiiiiii=1)^n i^3=((n(n+1))/2)^2";
        b.iter(|| {
            let mut tokenizer = Tokenizer::new(expression.to_string());
            let _ = tokenizer.parse();
        });
    }
}
