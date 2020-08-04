use crate::elements::Element;
use crate::tokens::Accent;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum ExpressionAccent {
    Generic(GenericAccent),
    OverSet(OverSet),
    UnderSet(UnderSet),
    Color(Color),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct GenericAccent {
    pub inner: Box<Element>,
    pub accent: Accent,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct OverSet {
    pub top: Box<Element>,
    pub bottom: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct UnderSet {
    pub top: Box<Element>,
    pub bottom: Box<Element>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Color {
    pub color: String,
    pub inner: Box<Element>,
}
