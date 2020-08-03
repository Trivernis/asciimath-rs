use crate::tokens::constants::operations::{
    G_AST, G_BACKSLASH, G_BIDWEDGE, G_BIGCAP, G_BIGCUP, G_BIGVEE, G_BOWTIE, G_CAP, G_CDOT, G_CIRC,
    G_CUP, G_DIV, G_LTIMES, G_MINUS, G_ODOT, G_OPLUS, G_PLUS, G_PROD, G_RTIMES, G_SLASH, G_STAR,
    G_SUM, G_TIMES, G_VEE, G_WEDGE,
};
use crate::tokens::mappings::get_operation_mappings;
use crate::tokens::{Operation, Token};
use charred::tapemachine::CharTapeMachine;
use std::collections::HashMap;

pub struct Tokenizer {
    ctm: CharTapeMachine,
    tokens: Vec<Token>,
}

impl Tokenizer {
    pub fn new(text: String) -> Self {
        Self {
            ctm: CharTapeMachine::new(text.chars().collect()),
            tokens: Vec::new(),
        }
    }

    fn parse_operation(&mut self) -> Option<Operation> {
        lazy_static! {
            static ref OPERATION_MAPPINGS: Vec<HashMap<&'static [&'static str], Operation>> =
                get_operation_mappings();
        }
        for mapping in OPERATION_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }
}
