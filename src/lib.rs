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
/// ```
/// let expression = asciimath_rs::parse("sin(2x) + 3".to_string());
/// ```
pub fn parse<S: AsRef<str>>(content: S) -> Expression {
    let mut tokenizer = Tokenizer::new(content);
    let tokens = tokenizer.parse();
    let mut tree_parser = TreeParser::new(tokens);

    tree_parser.parse()
}

#[cfg(test)]
mod tests;
