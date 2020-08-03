use crate::tokens::constants::accents::*;
use crate::tokens::constants::arrows::*;
use crate::tokens::constants::font_commands::*;
use crate::tokens::constants::greek::*;
use crate::tokens::constants::grouping::*;
use crate::tokens::constants::logical::*;
use crate::tokens::constants::misc::*;
use crate::tokens::constants::operations::*;
use crate::tokens::constants::relations::*;

use crate::tokens::constants::TokenPattern;
use crate::tokens::{
    Accent, Arrow, FontCommand, Greek, Grouping, Logical, Misc, Operation, Relation,
};
use std::cell::RefCell;
use std::collections::HashMap;

pub fn get_operation_mappings() -> Vec<HashMap<TokenPattern, Operation>> {
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

pub fn get_misc_mappings() -> Vec<HashMap<TokenPattern, Misc>> {
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

pub fn get_relation_mapping() -> Vec<HashMap<TokenPattern, Relation>> {
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

pub fn get_logical_mappings() -> Vec<HashMap<TokenPattern, Logical>> {
    vec![
        hashmap! {
            G_IFF => Logical::Iff,
        },
        hashmap! {
            G_AND => Logical::And,
            G_OR => Logical::Or,
            G_NOT => Logical::Not,
            G_IMPLIES => Logical::Implies,
            G_IF => Logical::If,
            G_FORALL => Logical::ForAll,
            G_EXISTS => Logical::Exists,
            G_BOT => Logical::Bot,
            G_TOP => Logical::Top,
            G_VDASH => Logical::VDash,
            G_MODELS => Logical::Models,
        },
    ]
}

pub fn get_grouping_mappings() -> Vec<HashMap<TokenPattern, Grouping>> {
    vec![
        hashmap! {
            G_LANGLE => Grouping::LAngle,
            G_RANGLE => Grouping::RAngle,
            G_RXPAR => Grouping::RXPar,
            G_LXPAR => Grouping::LXPar,
        },
        hashmap! {
            G_RPAREN => Grouping::RParen,
            G_LPAREN => Grouping::LParen,
            G_RBRAC => Grouping::RBrace,
            G_LBRAC => Grouping::LBrace,
            G_RCURL => Grouping::RCurl,
            G_LCURL => Grouping::LCurl,
            G_ABS => Grouping::Abs,
            G_FLOOR => Grouping::Floor,
            G_CEIL => Grouping::Ceil,
            G_NORM => Grouping::Norm,
        },
    ]
}

pub fn get_arrow_mapping() -> Vec<HashMap<TokenPattern, Arrow>> {
    vec![
        hashmap! {
            G_TWOHEADRIGHTARROW => Arrow::TwoHeadRightArrow,
            G_TWOHEADRIGHTARROWTAIL => Arrow::TwoHeadRightArrowTail
        },
        hashmap! {
            G_UPARROW => Arrow::UpArrow,
            G_DOWNARROW => Arrow::DownArrow,
            G_RIGHTARROW => Arrow::RightArrow,
            G_TO => Arrow::To,
            G_RIGHTARROWTAIL => Arrow::RightArrowTail,
            G_MAPSTO => Arrow::MapsTo,
            G_LEFTARROW => Arrow::LeftArrow,
            G_LEFTRIGHTARROW => Arrow::LeftRightArrow,
            G_BIGRIGHTARROW => Arrow::BigRightArrow,
            G_BIGLEFTARROW => Arrow::BigLeftArrow,
            G_BIGLEFTRIGHTARROW => Arrow::BigLeftRightArrow,
        },
    ]
}

pub fn get_accent_mappings() -> Vec<HashMap<TokenPattern, Accent>> {
    vec![hashmap! {
        G_HAT => Accent::Hat,
        G_UNDERLINE => Accent::Underline,
        G_OVERLINE => Accent::Overline,
        G_VEC => Accent::Vec,
        G_DOT => Accent::Dot,
        G_DDOT => Accent::DDot,
        G_OVERSET => Accent::OverSet,
        G_UNDERSET => Accent::UnderSet,
        G_UNDERBRACE => Accent::UnderBrace,
        G_OVERBRACE => Accent::OverBrace,
        G_COLOR => Accent::Color,
        G_CANCEL => Accent::Cancel,
    }]
}

pub fn get_greek_mappings() -> Vec<HashMap<TokenPattern, Greek>> {
    vec![hashmap! {
        G_ALPHA => Greek::Alpha,
        G_BETA => Greek::Beta,
        G_GAMMA => Greek::Gamma,
        G_BIGGAMMA => Greek::BigGamma,
        G_DELTA => Greek::Delta,
        G_BIGDELTA => Greek::BigDelta,
        G_EPSILON => Greek::Epsilon,
        G_VAREPSILON => Greek::VarEpsilon,
        G_ZETA => Greek::Zeta,
        G_ETA => Greek::Eta,
        G_THETA => Greek::Theta,
        G_BIGTHETA => Greek::BigTheta,
        G_VARTHETA => Greek::VarTheta,
        G_IOTA => Greek::Iota,
        G_KAPPA => Greek::Kappa,
        G_LAMBDA => Greek::Lambda,
        G_BIGLAMBDA => Greek::BigLambda,
        G_MU => Greek::Mu,
        G_NU => Greek::Nu,
        G_XI => Greek::Xi,
        G_BIGXI => Greek::BigXi,
        G_PI => Greek::Pi,
        G_BIGPI => Greek::BigPi,
        G_RHO => Greek::Rho,
        G_SIGMA => Greek::Sigma,
        G_BIGSIGMA => Greek::BigSigma,
        G_TAU => Greek::Tau,
        G_UPSILON => Greek::Upsilon,
        G_PHI => Greek::Phi,
        G_BIGPHI => Greek::BigPhi,
        G_VARPHI => Greek::VarPhi,
        G_CHI => Greek::Chi,
        G_PSI => Greek::Psi,
        G_BIGPSI => Greek::BigPsi,
        G_OMEGA => Greek::Omega,
        G_BIGOMEGA => Greek::BigOmega,
    }]
}

pub fn get_font_mappings() -> Vec<HashMap<&'static str, FontCommand>> {
    vec![
        hashmap! {
            F_BBB => FontCommand::BigOutline
        },
        hashmap! {
            F_BB => FontCommand::Big,
            F_CC => FontCommand::Cursive,
            F_TT => FontCommand::TText,
            F_FR => FontCommand::Fr,
            F_SF => FontCommand::SansSerif,
        },
    ]
}
