use crate::format::mathml::ToMathML;
use crate::parse;

#[test]
fn it_renders_roots() {
    let expr = parse("root 3 16".to_string());
    assert_eq!(
        expr.to_mathml(),
        "<mrow><mroot><mn>16</mn><mn>3</mn></mroot></mrow>"
    )
}
