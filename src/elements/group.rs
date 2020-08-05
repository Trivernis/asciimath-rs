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
