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

    #[test]
    fn it_works() {
        let expression = "sum_(i=1)^n";
        let mut tokenizer = Tokenizer::new(expression.to_string());
        let tokens = tokenizer.parse();
        assert_eq!(
            tokens,
            vec![
                Token::Operation(Operation::Sum),
                Token::Misc(Misc::Sub),
                Token::Grouping(Grouping::RParen),
                Token::Text(Text::Plain("i".to_string())),
                Token::Relation(Relation::Eq),
                Token::Text(Text::Plain("1".to_string())),
                Token::Grouping(Grouping::LParen),
                Token::Misc(Misc::Pow),
                Token::Text(Text::Plain("n".to_string()))
            ]
        );
    }
}
