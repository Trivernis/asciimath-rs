use crate::elements::accent::ExpressionAccent;
use crate::elements::group::Group;
use crate::elements::literal::Literal;
use crate::elements::special::Special;
use crate::utils::Boxed;

pub mod accent;
pub mod group;
pub mod literal;
pub mod special;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Element {
    Literal(Literal),
    Special(Special),
    Group(Group),
    Accent(ExpressionAccent),
    Null,
}

impl Boxed for Element {}

impl Element {
    pub fn to_non_enclosed(&self) -> Self {
        if let Element::Group(g) = self {
            if let Some(ne) = g.to_non_enclosed() {
                Element::Group(ne)
            } else {
                Element::Group(g.clone())
            }
        } else {
            self.clone()
        }
    }
}
