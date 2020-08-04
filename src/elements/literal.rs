use crate::tokens::{Arrow, FontCommand, Function, Greek, Logical, Misc, Operation, Relation};

#[derive(Clone, Debug)]
pub enum Literal {
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
    Misc(Misc),
    Operation(Operation),
}

#[derive(Clone, Debug)]
pub struct TextNode {
    pub(crate) text: String,
}

#[derive(Clone, Debug)]
pub struct SymbolNode {
    pub(crate) symbol: String,
}

#[derive(Clone, Debug)]
pub struct NumberNode {
    pub(crate) number: String,
}
