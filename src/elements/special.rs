use crate::elements::Element;

#[derive(Clone, Debug)]
pub struct Expression {
    children: Vec<Element>,
}

#[derive(Clone, Debug)]
pub enum Special {
    Sum(Sum),
    Prod(Prod),
    Frac(Frac),
    Exp(Exp),
    Sqrt(Sqrt),
    Root(Root),
    Integral(Integral),
    OIntegral(OIntegral),
}

#[derive(Clone, Debug)]
pub struct Sum {
    top: Option<Expression>,
    bottom: Option<Expression>,
}

#[derive(Clone, Debug)]
pub struct Prod {
    top: Box<Expression>,
    bottom: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Frac {
    top: Box<Expression>,
    bottom: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Exp {
    base: Box<Element>,
    exp: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Sqrt {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Root {
    base: Box<Expression>,
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Integral {
    top: Box<Expression>,
    bottom: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct OIntegral {
    top: Box<Expression>,
    bottom: Box<Expression>,
}
