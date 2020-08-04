use crate::elements::group::Group;
use crate::elements::literal::Literal;
use crate::elements::special::Special;

pub mod group;
pub mod literal;
pub mod special;

#[derive(Debug, Clone)]
pub enum Element {
    Literal(Literal),
    Special(Special),
    Group(Group),
}
