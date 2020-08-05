use crate::elements::literal::{Number, PlainText, Symbol};
use crate::tokens::{Arrow, FontCommand, Function, Greek, Logical, Misc, Relation};
use htmlescape::encode_minimal;

pub trait ToMathML {
    fn to_mathml(&self) -> String;
}

impl ToMathML for Greek {
    fn to_mathml(&self) -> String {
        let inner = match self {
            Greek::Alpha => "&alpha;",
            Greek::Beta => "&beta;",
            Greek::Gamma => "&gamma;",
            Greek::BigGamma => "&Gamma;",
            Greek::Delta => "&delta;",
            Greek::BigDelta => "&Delta;",
            Greek::Epsilon => "&epsilon;",
            Greek::VarEpsilon => "&epsilon;",
            Greek::Zeta => "&zeta;",
            Greek::Eta => "&eta;",
            Greek::Theta => "&theta;",
            Greek::BigTheta => "&Theta;",
            Greek::VarTheta => "&theta;",
            Greek::Iota => "&iota;",
            Greek::Kappa => "&kappa;",
            Greek::Lambda => "&lambda;",
            Greek::BigLambda => "&Lambda;",
            Greek::Mu => "&mu;",
            Greek::Nu => "&nu;",
            Greek::Xi => "&xi;",
            Greek::BigXi => "&Xi;",
            Greek::Pi => "&pi;",
            Greek::BigPi => "&Pi;",
            Greek::Rho => "&rho;",
            Greek::Sigma => "&sigma;",
            Greek::BigSigma => "&Sigma;",
            Greek::Tau => "&tau;",
            Greek::Upsilon => "&upsilon;",
            Greek::Phi => "&phi;",
            Greek::BigPhi => "&Phi;",
            Greek::VarPhi => "&phi;",
            Greek::Chi => "&chi;",
            Greek::Psi => "&psi;",
            Greek::BigPsi => "&Psi;",
            Greek::Omega => "&omega;",
            Greek::BigOmega => "&Omega;",
        };

        format!("<mi>{}</mi>", inner)
    }
}

impl ToMathML for PlainText {
    fn to_mathml(&self) -> String {
        if let Some(formatting) = &self.formatting {
            format!(
                "<mtext mathvariant='{}'/>{}</mtext>",
                formatting.to_mathml(),
                encode_minimal(self.text.as_str())
            )
        } else {
            format!("<mtext/>{}</mtext>", encode_minimal(self.text.as_str()))
        }
    }
}

impl ToMathML for FontCommand {
    fn to_mathml(&self) -> String {
        match self {
            FontCommand::Big => "bold".to_string(),
            FontCommand::BigOutline => "double-struck",
            FontCommand::Cursive => "italic",
            FontCommand::TText => "script",
            FontCommand::Fr => "bold-fraktur",
            FontCommand::SansSerif => "sans-serif",
        }
    }
}

impl ToMathML for Symbol {
    fn to_mathml(&self) -> String {
        format!("<mi>{}</mi>", encode_minimal(self.symbol.as_str()))
    }
}

impl ToMathML for Number {
    fn to_mathml(&self) -> String {
        format!("<mn>{}</mn>", encode_minimal(self.number.as_str()))
    }
}

impl ToMathML for Relation {
    fn to_mathml(&self) -> String {
        let inner = match self {
            Relation::Eq => "=",
            Relation::Ne => "&ne;",
            Relation::Lt => "&lt;",
            Relation::Gt => "&gt;",
            Relation::Le => "&le;",
            Relation::Ge => "&ge;",
            Relation::Prec => "&pr;",
            Relation::Succ => "&sc;",
            Relation::PrecEq => "&prcue;",
            Relation::SuccEq => "&sccue;",
            Relation::In => "&isin;",
            Relation::NotIn => "&notin;",
            Relation::SubSet => "&sub;",
            Relation::SupSet => "&sup;",
            Relation::SubSetEq => "&sube;",
            Relation::SupSetEq => "&supe;",
            Relation::Equiv => "&equiv;",
            Relation::Cong => "&cong;",
            Relation::Approx => "&asymp;",
            Relation::PropTo => "&prop;",
        };

        format!("<mo>{}</mo>", inner)
    }
}

impl ToMathML for Function {
    fn to_mathml(&self) -> String {
        let inner = match self {
            Function::Exp => "exp",
            Function::Sin => "sin",
            Function::Max => "max",
            Function::Min => "min",
            Function::Glb => "glb",
            Function::G => "g",
            Function::Lub => "lub",
            Function::Lcm => "lcm",
            Function::Gcd => "gcd",
            Function::Mod => "mod",
            Function::Dim => "dim",
            Function::Det => "det",
            Function::Ln => "ln",
            Function::Log => "log",
            Function::Cot => "cot",
            Function::Csc => "csc",
            Function::Sech => "sech",
            Function::Tanh => "tanh",
            Function::Cosh => "cosh",
            Function::ArcSin => "arcsin",
            Function::ArcCos => "arccos",
            Function::ArcTan => "arctan",
            Function::Tan => "tan",
            Function::Cos => "cos",
            Function::F => "f",
            Function::Sec => "sec",
            Function::Sinh => "sinh",
            Function::Csch => "csch",
            Function::Coth => "coth",
        };
        format!("<mi>{}</mi>", inner)
    }
}

impl ToMathML for Logical {
    fn to_mathml(&self) -> String {
        let inner = match self {
            Logical::And => "and",
            Logical::Or => "or",
            Logical::Not => "&not;",
            Logical::Implies => "&rArr;",
            Logical::If => "if",
            Logical::Iff => "&hArr;",
            Logical::ForAll => "&forall;",
            Logical::Exists => "exists;",
            Logical::Bot => "&perp;",
            Logical::Top => "&top;",
            Logical::VDash => "&vdash;",
            Logical::Models => "&vDash;",
        };
        format!("<mo>{}</mo>", inner)
    }
}

impl ToMathML for Arrow {
    fn to_mathml(&self) -> String {
        let inner = match self {
            Arrow::UpArrow => "&#8593;",
            Arrow::DownArrow => "&#8595;",
            Arrow::RightArrow => "&#8594;",
            Arrow::To => "&#8594;",
            Arrow::RightArrowTail => "&#8611;",
            Arrow::TwoHeadRightArrow => "&#8608;",
            Arrow::TwoHeadRightArrowTail => "&Rarrtl;",
            Arrow::MapsTo => "&#8614;",
            Arrow::LeftArrow => "&#8592;",
            Arrow::LeftRightArrow => "&#10231;",
            Arrow::BigRightArrow => "&#8680;",
            Arrow::BigLeftArrow => "&#8678;",
            Arrow::BigLeftRightArrow => "&#11012;",
        };
        format!("<mo>{}</mo>", inner)
    }
}

impl ToMathML for Misc {
    fn to_mathml(&self) -> String {
        let inner = match self {
            Misc::Del => "&part;",
            Misc::Grad => "&nabla;",
            Misc::PlusMinus => "&plusmn;",
            Misc::EmptySet => "&empty;",
            Misc::Infty => "&infin;",
            Misc::Aleph => "&alefsym;",
            Misc::Therefore => "&there4;",
            Misc::Because => "&because;",
            Misc::PLDots => "|&hellip;|",
            Misc::PCDots => "|&middot;&middot;&middot;|",
            Misc::VDots => "&#65049;",
            Misc::DDots => "&dtdot;",
            Misc::EPipes => "||",
            Misc::EQuad => "| |",
            Misc::Angle => "&ang;",
            Misc::Frown => "&#8994;",
            Misc::Triangle => "&#9651;",
            Misc::Diamond => "&diamond;",
            Misc::Square => "&#9633;",
            Misc::LFloor => "&lfloor;",
            Misc::RFloor => "&rfloor;",
            Misc::LCeiling => "&lceil;",
            Misc::RCeiling => "&rceil;",
            Misc::Complex => "&Copf;",
            Misc::Natural => "&Nopf;",
            Misc::Rational => "&Qopf;",
            Misc::Real => "&Ropf;",
            Misc::Integer => "&Zopf;",
            _ => "",
        };

        format!("<mi>{}</mi>", inner)
    }
}
