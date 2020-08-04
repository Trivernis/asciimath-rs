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
    Pow(Pow),
    Sqrt(Sqrt),
    Root(Root),
    Integral(Integral),
    OIntegral(OIntegral),
}

#[derive(Clone, Debug)]
pub struct Sum {
    pub top: Option<Box<Element>>,
    pub bottom: Option<Box<Element>>,
}

#[derive(Clone, Debug)]
pub struct Prod {
    pub top: Option<Box<Element>>,
    pub bottom: Option<Box<Element>>,
}

#[derive(Clone, Debug)]
pub struct Frac {
    pub(crate) top: Box<Element>,
    pub(crate) bottom: Box<Element>,
}

#[derive(Clone, Debug)]
pub struct Pow {
    pub(crate) exp: Box<Element>,
}

#[derive(Clone, Debug)]
pub struct Sqrt {
    pub(crate) inner: Box<Element>,
}

#[derive(Clone, Debug)]
pub struct Root {
    pub(crate) base: Box<Element>,
    pub(crate) inner: Box<Element>,
}

#[derive(Clone, Debug)]
pub struct Integral {
    pub(crate) top: Option<Box<Element>>,
    pub(crate) bottom: Option<Box<Element>>,
}

#[derive(Clone, Debug)]
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
