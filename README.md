# AsciiMath ![](https://img.shields.io/discord/729250668162056313)

This project aims to implement a fully functional AsciiMath parser for rust.
It's part of the [snekdown](https://github.com/trivernis/snekdown) parser project.

See [the spec](http://asciimath.org/).

## Dependencies

- [charred](https://crates.io/crates/charred) used by the tokenizer to analyze the input string
- [maplit](https://crates.io/crates/maplit) for an easy to use macro to define the token mappings
- [lazy_static](https://crates.io/crates/lazy_static) to define static mappings for tokens
- [htmlescape](https://crates.io/crates/htmlescape) for escaping html when converting to mathml

## Usage

### The simple way

```rust
use asciimath_rs::format::mathml::ToMathML;

fn main() {
    let expression = asciimath_rs::parse("sin(2x) + 3".to_string());
    let mathml_string = expression.to_mathml();
}
```

### The less simple way

```rust
use asciimath_rs::parsing::tokenizer::Tokenizer;
use asciimath_rs::parsing::tree_parser::TreeParser;
use asciimath_rs::format::mathml::ToMathML;

fn main() {
    let mut tokenizer = Tokenizer::new("cos(2) - alpha".to_string());
    let tokens = tokenizer.parse();
    let mut tree_parser = TreeParser::new(tokens);
    let expression = tree_parser.parse();
    let mathml_string = expression.to_mathml();
}
```

## How it works

As seen in the less simple example the parsing works in two steps.
In the first step the raw input string is analyzed and converted into Tokens that represent
the syntactic meaning of a sequence of characters. 
The second step takes the flat vector of tokens and converts it into a tree in a depth first way.

The resulting expression can then be converted into MathML with the default `ToMathML` trait implementation.

## License

This project is [Apache 2.0](https://github.com/Trivernis/asciimath-rs/blob/main/LICENSE) licensed.