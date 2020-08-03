use crate::tokens::constants::operations::*;
use crate::tokens::Operation;
use std::collections::HashMap;

pub fn get_operation_mappings() -> Vec<HashMap<&'static [&'static str], Operation>> {
    vec![
        hashmap! {
            G_STAR => Operation::Star,
        },
        hashmap! {
            G_AST => Operation::Ast,
            G_BOWTIE => Operation::Bowtie,
            G_DIV => Operation::Div,
            G_BIDWEDGE => Operation::BidWedge,
            G_BIGVEE => Operation::BigVee,
            G_BIGCAP => Operation::BigCap,
            G_BIGCUP => Operation::BigCup,
        },
        hashmap! {
            G_PLUS => Operation::Plus,
            G_MINUS => Operation::Minus,
            G_CDOT => Operation::CDot,
            G_SLASH => Operation::Slash,
            G_BACKSLASH => Operation::Backslash,
            G_TIMES => Operation::Times,
            G_LTIMES => Operation::LTimes,
            G_RTIMES => Operation::RTimes,
            G_CIRC => Operation::Circ,
            G_OPLUS => Operation::OPlus,
            G_OTIMES => Operation::OTimes,
            G_ODOT => Operation::ODot,
            G_SUM => Operation::Sum,
            G_PROD => Operation::Prod,
            G_WEDGE => Operation::Wedge,
            G_VEE => Operation::Vee,
            G_CAP => Operation::Cap,
            G_CUP => Operation::Cup,
        },
    ]
}
