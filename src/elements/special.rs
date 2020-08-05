use crate::elements::Element;
use crate::utils::Boxed;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
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

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Sum {
    pub top: Option<Box<Element>>,
    pub bottom: Option<Box<Element>>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Prod {
    pub top: Option<Box<Element>>,
    pub bottom: Option<Box<Element>>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Frac {
    pub(crate) top: Box<Element>,
    pub(crate) bottom: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Pow {
    pub(crate) base: Box<Element>,
    pub(crate) exp: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Sub {
    pub(crate) base: Box<Element>,
    pub(crate) lower: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Sqrt {
    pub(crate) inner: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Root {
    pub(crate) base: Box<Element>,
    pub(crate) inner: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Integral {
    pub(crate) top: Option<Box<Element>>,
    pub(crate) bottom: Option<Box<Element>>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OIntegral {
    pub(crate) top: Option<Box<Element>>,
    pub(crate) bottom: Option<Box<Element>>,
}

impl Expression {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: Element) {
        self.children.push(child)
    }
}

impl Boxed for Expression {}

impl Sum {
    pub fn new() -> Self {
        Self {
            bottom: None,
            top: None,
        }
    }
}

impl Prod {
    pub fn new() -> Self {
        Self {
            bottom: None,
            top: None,
        }
    }
}
