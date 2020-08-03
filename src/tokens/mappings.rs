use std::collections::HashMap;
use crate::tokens::Operation;
use crate::tokens::constants::operations::*;

pub fn get_operation_mapping() -> HashMap<&'static[&'static str], Operation> {
    hashmap! {
        G_PLUS => Operation::Plus,
        G_MINUS => Operation::Minus,
        G_CDOT => Operation::CDot,
        G_AST => Operation::Ast,
        G_STAR => Operation::Star,
        G_SLASH => Operation::Slash,
        G_BACKSLASH => Operation::Backslash,
        G_TIMES => Operation::Times,
        G_DIV => Operation::Div,
        G_LTIMES => Operation::LTimes,
        G_RTIMES => Operation::RTimes,
        G_BOWTIE => Operation::Bowtie,
        G_CIRC => Operation::Circ,
        G_OPLUS => Operation::OPlus,
        G_OTIMES => Operation::OTimes,
        G_ODOT => Operation::ODot,
        G_SUM => Operation::Sum,
        G_PROD => Operation::Prod,
        G_WEDGE => Operation::Wedge,
        G_BIDWEDGE => Operation::BidWedge,
        G_VEE => Operation::Vee,
        G_BIGVEE => Operation::BigVee,
        G_CAP => Operation::Cap,
        G_BIGCAP => Operation::BigCap,
        G_CUP => Operation::Cup,
        G_BIGCUP => Operation::BigCup,
    }
}