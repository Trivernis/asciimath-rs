use crate::tokens::{Arrow, FontCommand, Function, Greek, Logical, Misc, Operation, Relation};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Literal {
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
    NewLine,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PlainText {
    pub(crate) text: String,
    pub(crate) formatting: Option<FontCommand>,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Symbol {
    pub(crate) symbol: String,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Number {
    pub(crate) number: String,
}
