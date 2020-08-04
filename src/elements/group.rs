use crate::elements::special::Expression;

#[derive(Clone, Debug)]
pub enum Group {
    Parentheses(Parentheses),
    Brackets(Brackets),
    Braces(Braces),
    Angles(Angles),
    XGroup(XGroup),
    Abs(Abs),
    Floor(Floor),
    Ceil(Ceil),
    Norm(Norm),
}

#[derive(Clone, Debug)]
pub struct Parentheses {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Brackets {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Braces {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Angles {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct XGroup {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Abs {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Floor {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Ceil {
    inner: Box<Expression>,
}

#[derive(Clone, Debug)]
pub struct Norm {
    inner: Box<Expression>,
}
