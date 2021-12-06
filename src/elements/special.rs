use crate::elements::Element;
use crate::utils::Boxed;

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct Expression {
    pub children: Vec<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Special {
    Sum(Sum),
    Prod(Prod),
    Frac(Frac),
    Pow(Pow),
    Sub(Sub),
    Sqrt(Sqrt),
    Root(Root),
    Integral(Integral),
    OIntegral(OIntegral),
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct Sum {
    pub top: Option<Box<Element>>,
    pub bottom: Option<Box<Element>>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Default)]
pub struct Prod {
    pub top: Option<Box<Element>>,
    pub bottom: Option<Box<Element>>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Frac {
    pub top: Box<Element>,
    pub bottom: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pow {
    pub base: Box<Element>,
    pub exp: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Sub {
    pub base: Box<Element>,
    pub lower: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Sqrt {
    pub inner: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Root {
    pub base: Box<Element>,
    pub inner: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Integral {
    pub top: Option<Box<Element>>,
    pub bottom: Option<Box<Element>>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OIntegral {
    pub top: Option<Box<Element>>,
    pub bottom: Option<Box<Element>>,
}

impl Expression {
    pub fn add_child(&mut self, child: Element) {
        self.children.push(child)
    }
}

impl Boxed for Expression {}
