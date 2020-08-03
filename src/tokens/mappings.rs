use crate::tokens::constants::misc::*;
use crate::tokens::constants::operations::*;
use crate::tokens::{Misc, Operation};
use std::cell::RefCell;
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

pub fn get_misc_mappings() -> Vec<HashMap<&'static [&'static str], Misc>> {
    vec![
        hashmap! {
            G_TRIANGLE => Misc::Triangle,
        },
        hashmap! {
            G_ANGLE => Misc::Angle,
        },
        hashmap! {
            G_A_FRAC => Misc::AsciiFrac,
            G_T_FRAC => Misc::LatexFrac,
            G_POW => Misc::Pow,
            G_SQRT => Misc::Sqrt,
            G_ROOT => Misc::Root,
            G_INT => Misc::Int,
            G_OINT => Misc::OInt,
            G_DEL => Misc::Del,
            G_GRAD => Misc::Grad,
            G_PM => Misc::PlusMinus,
            G_EMPTYSET => Misc::EmptySet,
            G_INFTY => Misc::Infty,
            G_ALEPH => Misc::Aleph,
            G_THEREFORE => Misc::Therefore,
            G_BECAUSE => Misc::Because,
            G_ELDOTS => Misc::PLDots,
            G_ECDOTS => Misc::PCDots,
            G_VDOTS => Misc::VDots,
            G_DDOTS => Misc::DDots,
            G_EPIPES => Misc::EPipes,
            G_QUAD => Misc::EQuad,
            G_FROWN => Misc::Frown,
            G_DIAMOND => Misc::Diamond,
            G_SQUARE => Misc::Square,
            G_LFLOOR => Misc::LFloor,
            G_RFLOOR => Misc::RFloor,
            G_LCEILING => Misc::LCeiling,
            G_RCEILING => Misc::RCeiling,
            G_COMPLEX => Misc::Complex,
            G_NATURAL => Misc::Natural,
            G_RATIONAL => Misc::Rational,
            G_REAL => Misc::Real,
            G_INTEGER => Misc::Integer,
            G_A_TEXT => Misc::AsciiText,
            G_T_TEX => Misc::LatexText,
        },
    ]
}
