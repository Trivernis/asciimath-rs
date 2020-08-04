use crate::tokens::{Arrow, FontCommand, Function, Greek, Logical, Misc, Operation, Relation};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Literal {
    Integer,
    Text(PlainText),
    Symbol(Symbol),
    Number(Number),
    Greek(Greek),
    FontCommand(FontCommand),
    Relation(Relation),
    Function(Function),
    Logical(Logical),
    Arrow(Arrow),
    Misc(Misc),
    Operation(Operation),
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PlainText {
    pub(crate) text: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Symbol {
    pub(crate) symbol: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Number {
    pub(crate) number: String,
}
