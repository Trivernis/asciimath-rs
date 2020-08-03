use charred::tapemachine::CharTapeMachine;
use crate::tokens::{Token, Operation};
use crate::tokens::constants::operations::{G_PLUS, G_MINUS, G_CDOT, G_AST, G_STAR, G_SLASH, G_BACKSLASH, G_TIMES, G_DIV, G_LTIMES, G_RTIMES, G_BOWTIE, G_CIRC, G_OPLUS, G_ODOT, G_SUM, G_PROD, G_WEDGE, G_BIDWEDGE, G_VEE, G_BIGVEE, G_CAP, G_BIGCAP, G_CUP, G_BIGCUP};
use std::collections::HashMap;
use crate::tokens::mappings::get_operation_mapping;

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
        lazy_static! { static ref OPERATION_MAPPING: HashMap<&'static[&'static str], Operation> = get_operation_mapping();}
        for key in OPERATION_MAPPING.keys() {
            if self.ctm.check_any_str_sequence(*key) {
                return Some(OPERATION_MAPPING[key].clone())
            }
        }
        None
    }
}