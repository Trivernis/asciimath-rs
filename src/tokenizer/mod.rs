use crate::tokens::mappings::{
    get_logical_mappings, get_misc_mappings, get_operation_mappings, get_relation_mapping,
};
use crate::tokens::{Logical, Misc, Operation, Relation, Token};
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

    fn parse_misc(&mut self) -> Option<Misc> {
        lazy_static! {
            static ref MISC_MAPPINGS: Vec<HashMap<&'static [&'static str], Misc>> =
                get_misc_mappings();
        }
        for mapping in MISC_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
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

    fn parse_relation(&mut self) -> Option<Relation> {
        lazy_static! {
            static ref RELATION_MAPPINGS: Vec<HashMap<&'static [&'static str], Relation>> =
                get_relation_mapping();
        }
        for mapping in RELATION_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }

    fn parse_logical(&mut self) -> Option<Logical> {
        lazy_static! {
            static ref LOGICAL_MAPPINGS: Vec<HashMap<&'static [&'static str], Logical>> =
                get_logical_mappings();
        }
        for mapping in LOGICAL_MAPPINGS.iter() {
            for key in mapping.keys() {
                if self.ctm.check_any_str_sequence(*key) {
                    return Some(mapping[key].clone());
                }
            }
        }
        None
    }
}
