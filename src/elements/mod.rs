use crate::elements::group::Group;
use crate::elements::literal::Literal;
use crate::elements::special::Special;
use crate::utils::Boxed;

pub mod group;
pub mod literal;
pub mod special;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Element {
    Literal(Literal),
    Special(Special),
    Group(Group),
}

impl Boxed for Element {}
