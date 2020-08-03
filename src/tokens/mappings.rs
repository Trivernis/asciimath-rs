use crate::tokens::constants::misc::*;
use crate::tokens::constants::operations::*;
use crate::tokens::constants::relations::*;
use crate::tokens::{Misc, Operation, Relation};
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

pub fn get_relation_mapping() -> Vec<HashMap<&'static [&'static str], Relation>> {
    vec![
        hashmap! {
            G_SUBSETEQ => Relation::SubSetEq,
            G_SUPSETEQ => Relation::SupSetEq,
            G_LE => Relation::Le,
            G_GE => Relation::Ge,
            G_SUCCEQ => Relation::SuccEq,
            G_PRECEQ => Relation::PrecEq,
        },
        hashmap! {
            G_SUCC => Relation::Succ,
        },
        hashmap! {
            G_EQ => Relation::Eq,
            G_NE => Relation::Ne,
            G_LT => Relation::Lt,
            G_GT => Relation::Gt,
            G_PREC => Relation::Prec,
            G_IN => Relation::In,
            G_NOTIN => Relation::NotIn,
            G_SUBSET => Relation::SubSet,
            G_SUPSET => Relation::SupSet,
            G_EQUIV => Relation::Equiv,
            G_CONG => Relation::Cong,
            G_APPROX => Relation::Approx,
            G_PROP => Relation::PropTo,
        },
    ]
}
