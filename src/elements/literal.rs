use crate::tokens::{Arrow, FontCommand, Function, Greek, Logical, Relation};

#[derive(Clone, Debug)]
pub enum Literal {
    Plus,
    Minus,
    CDot,
    Ast,
    Star,
    Slash,
    Backslash,
    Times,
    Div,
    LTimes,
    RTimes,
    Bowtie,
    Circ,
    OPlus,
    OTimes,
    ODot,
    Wedge,
    BidWedge,
    Vee,
    Cap,
    BigCap,
    Cup,
    BigCup,
    Del,
    Grad,
    PlusMinus,
    EmptySet,
    Infty,
    Aleph,
    Therefore,
    Because,
    LDots,
    CDots,
    VDots,
    DDots,
    EPipes,
    Quad,
    Angle,
    Frown,
    Triangle,
    Diamond,
    Square,
    LFloor,
    RFloor,
    LCeiling,
    RCeiling,
    Complex,
    Natural,
    Rational,
    Real,
    Integer,
    Text(TextNode),
    Symbol(SymbolNode),
    Number(NumberNode),
    Greek(Greek),
    FontCommand(FontCommand),
    Relation(Relation),
    Function(Function),
    Logical(Logical),
    Arrow(Arrow),
}

#[derive(Clone, Debug)]
pub struct TextNode {
    text: String,
}

#[derive(Clone, Debug)]
pub struct SymbolNode {
    symbol: String,
}

#[derive(Clone, Debug)]
pub struct NumberNode {
    number: String,
}
