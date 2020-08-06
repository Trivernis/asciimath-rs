use crate::elements::special::Expression;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Group {
    MSep,
    Parentheses(Parentheses),
    Brackets(Brackets),
    Braces(Braces),
    Angles(Angles),
    XGroup(XGroup),
    Abs(Abs),
    Floor(Floor),
    Ceil(Ceil),
    Norm(Norm),
    Matrix(Matrix),
    Vector(Vector),
    NonEnclosed(NonEnclosed),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Parentheses {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Brackets {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Braces {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Angles {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct XGroup {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Abs {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Floor {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Ceil {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Norm {
    pub inner: Box<Expression>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Matrix {
    pub inner: Vec<Vec<Expression>>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Vector {
    pub inner: Vec<Vec<Expression>>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct NonEnclosed {
    pub inner: Box<Expression>,
}

impl Group {
    pub(crate) fn to_non_enclosed(&self) -> Option<Self> {
        let inner = match self {
            Group::Parentheses(p) => Some(p.inner.clone()),
            Group::Braces(b) => Some(b.inner.clone()),
            Group::Brackets(b) => Some(b.inner.clone()),
            _ => None,
        };
        if let Some(inner) = inner {
            Some(Group::NonEnclosed(NonEnclosed { inner }))
        } else {
            None
        }
    }
}
