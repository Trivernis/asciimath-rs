use crate::elements::accent::{Color, ExpressionAccent, GenericAccent, OverSet, UnderSet};
use crate::elements::group::{
    Abs, Angles, Braces, Brackets, Ceil, Floor, Group, Matrix, NonEnclosed, Norm, Parentheses,
    Vector, XGroup,
};
use crate::elements::literal::{Literal, Number, PlainText, Symbol};
use crate::elements::special::{
    Expression, Frac, Integral, OIntegral, Pow, Prod, Root, Special, Sqrt, Sub, Sum,
};
use crate::elements::Element;
use crate::tokens::{
    Accent, Arrow, FontCommand, Function, Greek, Logical, Misc, Operation, Relation,
};
use htmlescape::{encode_attribute, encode_minimal};

/// Trait to convert the given object into a MathML representation.
pub trait ToMathML {
    fn to_mathml(&self) -> String;
}

impl ToMathML for Literal {
    fn to_mathml(&self) -> String {
        match self {
            Literal::Text(t) => t.to_mathml(),
            Literal::Symbol(s) => s.to_mathml(),
            Literal::Number(n) => n.to_mathml(),
            Literal::Greek(g) => g.to_mathml(),
            Literal::FontCommand(f) => f.to_mathml(),
            Literal::Relation(r) => r.to_mathml(),
            Literal::Function(f) => f.to_mathml(),
            Literal::Logical(l) => l.to_mathml(),
            Literal::Arrow(a) => a.to_mathml(),
            Literal::Misc(m) => m.to_mathml(),
            Literal::Operation(o) => o.to_mathml(),
            Literal::NewLine => "<mspace linebreak='newline' />".to_string(),
        }
    }
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
                "<mtext mathvariant='{}'>{}</mtext>",
                formatting.to_mathml(),
                encode_minimal(self.text.as_str())
            )
        } else {
            format!("<mtext>{}</mtext>", encode_minimal(self.text.as_str()))
        }
    }
}

impl ToMathML for FontCommand {
    fn to_mathml(&self) -> String {
        match self {
            FontCommand::Big => "bold".to_string(),
            FontCommand::BigOutline => "double-struck".to_string(),
            FontCommand::Cursive => "italic".to_string(),
            FontCommand::TText => "script".to_string(),
            FontCommand::Fr => "bold-fraktur".to_string(),
            FontCommand::SansSerif => "sans-serif".to_string(),
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

impl ToMathML for Operation {
    fn to_mathml(&self) -> String {
        let inner = match self {
            Operation::Plus => "&plus;",
            Operation::Minus => "&minus;",
            Operation::CDot => "&sdot;",
            Operation::Ast => "&lowast;",
            Operation::Star => "&Star;",
            Operation::Slash => "/",
            Operation::Backslash => "&setminus;",
            Operation::Times => "&times;",
            Operation::Div => "&divide;",
            Operation::LTimes => "&ltimes;",
            Operation::RTimes => "&rtimes;",
            Operation::Bowtie => "&bowtie;",
            Operation::Circ => "&compfn;",
            Operation::OPlus => "&oplus;",
            Operation::OTimes => "&otimes;",
            Operation::ODot => "&odot;",
            Operation::Wedge => "&and;",
            Operation::BidWedge => "&xwedge;",
            Operation::Vee => "&or;",
            Operation::BigVee => "&xvee;",
            Operation::Cap => "&cap;",
            Operation::BigCap => "&xcap;",
            Operation::Cup => "&cup;",
            Operation::BigCup => "&xcup;",
            _ => "",
        };
        format!("<mo>{}</mo>", inner)
    }
}

impl ToMathML for Accent {
    fn to_mathml(&self) -> String {
        match self {
            Accent::Hat => "&circ;".to_string(),
            Accent::Overline => "&macr;".to_string(),
            Accent::Underline => "&ndash;".to_string(),
            Accent::Vec => "&#8594;".to_string(),
            Accent::Dot => ".".to_string(),
            Accent::DDot => "..".to_string(),
            Accent::UnderBrace => "&#9183;".to_string(),
            Accent::OverBrace => "&#9182;".to_string(),
            Accent::Cancel => "&#10187;".to_string(),
            _ => "".to_string(),
        }
    }
}

impl ToMathML for OverSet {
    fn to_mathml(&self) -> String {
        format!(
            "<mover accentover='true'><mrow>{}</mrow><mo>{}</mo>",
            self.bottom.to_mathml(),
            self.top.to_mathml()
        )
    }
}

impl ToMathML for UnderSet {
    fn to_mathml(&self) -> String {
        format!(
            "<munder accentunder='true'><mrow>{}</mrow><mo>{}</mo>",
            self.top.to_mathml(),
            self.bottom.to_mathml(),
        )
    }
}

impl ToMathML for Color {
    fn to_mathml(&self) -> String {
        format!(
            "<mstyle mathcolor='{}'>{}</mstyle>",
            encode_attribute(self.color.as_str()),
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for GenericAccent {
    fn to_mathml(&self) -> String {
        match self.accent {
            Accent::Hat
            | Accent::Overline
            | Accent::Vec
            | Accent::Dot
            | Accent::DDot
            | Accent::OverBrace => format!(
                "<mover accentover='true'><mrow>{}</mrow><mo>{}</mo></mover>",
                self.inner.to_mathml(),
                self.accent.to_mathml()
            ),
            Accent::Underline | Accent::UnderBrace => format!(
                "<munder accentunder='true'><mrow>{}</mrow><mo>{}</mo></mover>",
                self.inner.to_mathml(),
                self.accent.to_mathml()
            ),
            _ => self.inner.to_mathml(),
        }
    }
}

impl ToMathML for Group {
    fn to_mathml(&self) -> String {
        match self {
            Group::Vector(v) => v.to_mathml(),
            Group::MSep => "<mo>,</mo>".to_string(),
            Group::Parentheses(p) => p.to_mathml(),
            Group::Brackets(b) => b.to_mathml(),
            Group::Braces(b) => b.to_mathml(),
            Group::Angles(a) => a.to_mathml(),
            Group::XGroup(x) => x.to_mathml(),
            Group::Abs(a) => a.to_mathml(),
            Group::Floor(f) => f.to_mathml(),
            Group::Ceil(c) => c.to_mathml(),
            Group::Norm(n) => n.to_mathml(),
            Group::Matrix(m) => m.to_mathml(),
            Group::NonEnclosed(ne) => ne.to_mathml(),
        }
    }
}

impl ToMathML for Parentheses {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>(</mo>{}<mo>)</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Brackets {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>[</mo>{}<mo>]</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Braces {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>&lbrace;</mo>{}<mo>&rbrace;</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Angles {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>&#10216;</mo>{}<mo>&#10217;</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for XGroup {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>(x</mo>{}<mo>x)</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Abs {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>|</mo>{}<mo>|</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Floor {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>&lfloor;</mo>{}<mo>&rfloor;</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Ceil {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>&lceil;</mo>{}<mo>&rceil;</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Norm {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>||</mo>{}<mo>||</mo></mrow>",
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Matrix {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>[</mo><mtable>{}</mtable><mo>]</mo></mrow>",
            self.inner.iter().fold("".to_string(), |a, b| format!(
                "{}<mtr>{}</mtr>",
                a,
                b.iter().fold("".to_string(), |a, b| format!(
                    "{}<mtd>{}</mtd>",
                    a,
                    b.to_mathml()
                ))
            ))
        )
    }
}

impl ToMathML for Vector {
    fn to_mathml(&self) -> String {
        format!(
            "<mrow><mo>(</mo><mtable>{}</mtable><mo>)</mo></mrow>",
            self.inner.iter().fold("".to_string(), |a, b| format!(
                "{}<mtr>{}</mtr>",
                a,
                b.iter().fold("".to_string(), |a, b| format!(
                    "{}<mtd>{}</mtd>",
                    a,
                    b.to_mathml()
                ))
            ))
        )
    }
}

impl ToMathML for NonEnclosed {
    fn to_mathml(&self) -> String {
        format!("<mrow>{}</mrow>", self.inner.to_mathml())
    }
}

impl ToMathML for Special {
    fn to_mathml(&self) -> String {
        match self {
            Special::Sum(s) => s.to_mathml(),
            Special::Prod(p) => p.to_mathml(),
            Special::Frac(f) => f.to_mathml(),
            Special::Pow(p) => p.to_mathml(),
            Special::Sub(s) => s.to_mathml(),
            Special::Sqrt(s) => s.to_mathml(),
            Special::Root(r) => r.to_mathml(),
            Special::Integral(i) => i.to_mathml(),
            Special::OIntegral(i) => i.to_mathml(),
        }
    }
}

impl ToMathML for Sum {
    fn to_mathml(&self) -> String {
        if let Some(bottom) = &self.bottom {
            if let Some(top) = &self.top {
                format!(
                    "<munderover><mi>&sum;</mi>{}{}</munderover>",
                    bottom.to_mathml(),
                    top.to_mathml()
                )
            } else {
                format!("<munder><mi>&sum;</mi>{}</munder>", bottom.to_mathml())
            }
        } else if let Some(top) = &self.top {
            format!("<mover><mi>&sum;<mi>{}</mover>", top.to_mathml())
        } else {
            format!("<mi>&sum;</mi>")
        }
    }
}

impl ToMathML for Prod {
    fn to_mathml(&self) -> String {
        if let Some(bottom) = &self.bottom {
            if let Some(top) = &self.top {
                format!(
                    "<munderover><mi>&prod;</mi>{}{}</munderover>",
                    bottom.to_mathml(),
                    top.to_mathml()
                )
            } else {
                format!("<munder><mi>&prod;</mi>{}</munder>", bottom.to_mathml())
            }
        } else if let Some(top) = &self.top {
            format!("<mover><mi>&prod;<mi>{}</mover>", top.to_mathml())
        } else {
            format!("<mi>&prod;</mi>")
        }
    }
}

impl ToMathML for Frac {
    fn to_mathml(&self) -> String {
        format!(
            "<mfrac>{}{}</mfrac>",
            self.top.to_mathml(),
            self.bottom.to_mathml()
        )
    }
}

impl ToMathML for Sqrt {
    fn to_mathml(&self) -> String {
        format!("<msqrt>{}</msqrt>", self.inner.to_mathml())
    }
}

impl ToMathML for Root {
    fn to_mathml(&self) -> String {
        format!(
            "<mroot>{}{}</mroot>",
            self.base.to_mathml(),
            self.inner.to_mathml()
        )
    }
}

impl ToMathML for Pow {
    fn to_mathml(&self) -> String {
        format!(
            "<msup>{}{}</msup>",
            self.base.to_mathml(),
            self.exp.to_mathml()
        )
    }
}

impl ToMathML for Sub {
    fn to_mathml(&self) -> String {
        format!(
            "<msub>{}{}</msub>",
            self.base.to_mathml(),
            self.lower.to_mathml()
        )
    }
}

impl ToMathML for Integral {
    fn to_mathml(&self) -> String {
        if let Some(bottom) = &self.bottom {
            if let Some(top) = &self.top {
                format!(
                    "<munderover><mi>&int;</mi>{}{}</munderover>",
                    bottom.to_mathml(),
                    top.to_mathml()
                )
            } else {
                format!("<munder><mi>&int;</mi>{}</munder>", bottom.to_mathml())
            }
        } else if let Some(top) = &self.top {
            format!("<mover><mi>&int;<mi>{}</mover>", top.to_mathml())
        } else {
            format!("<mi>&int;</mi>")
        }
    }
}

impl ToMathML for OIntegral {
    fn to_mathml(&self) -> String {
        if let Some(bottom) = &self.bottom {
            if let Some(top) = &self.top {
                format!(
                    "<munderover><mi>&conint;</mi>{}{}</munderover>",
                    bottom.to_mathml(),
                    top.to_mathml()
                )
            } else {
                format!("<munder><mi>&conint;</mi>{}</munder>", bottom.to_mathml())
            }
        } else if let Some(top) = &self.top {
            format!("<mover><mi>&conint;<mi>{}</mover>", top.to_mathml())
        } else {
            format!("<mi>&conint;</mi>")
        }
    }
}

impl ToMathML for ExpressionAccent {
    fn to_mathml(&self) -> String {
        match self {
            ExpressionAccent::Generic(g) => g.to_mathml(),
            ExpressionAccent::OverSet(o) => o.to_mathml(),
            ExpressionAccent::UnderSet(u) => u.to_mathml(),
            ExpressionAccent::Color(c) => c.to_mathml(),
        }
    }
}

impl ToMathML for Expression {
    /// Recursively converts the Expression into a MathML representation.
    ///
    /// The result needs to be enclosed in `<math></math>` elements when used within
    /// an html file. Currently MathML is only natively supported by Firefox and Safari.
    ///
    /// Example:
    ///
    ///```
    /// use asciimath_rs::format::mathml::ToMathML;
    ///
    /// fn main() {
    ///     let expression = asciimath_rs::parse("sin(2x - 1) + 2".to_string());
    ///     println!("<math>{}</math>", expression.to_mathml());
    /// }
    /// ```
    fn to_mathml(&self) -> String {
        format!(
            "<mrow>{}</mrow>",
            self.children
                .iter()
                .fold("".to_string(), |a, b| format!("{}{}", a, b.to_mathml()))
        )
    }
}

impl ToMathML for Element {
    fn to_mathml(&self) -> String {
        match self {
            Element::Special(s) => s.to_mathml(),
            Element::Literal(l) => l.to_mathml(),
            Element::Group(g) => g.to_mathml(),
            Element::Accent(a) => a.to_mathml(),
        }
    }
}
