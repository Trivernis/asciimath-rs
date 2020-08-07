pub mod constants;
pub mod mappings;

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Token {
    Operation(Operation),
    Misc(Misc),
    Relation(Relation),
    Logical(Logical),
    Grouping(Grouping),
    Arrow(Arrow),
    Accent(Accent),
    Greek(Greek),
    Font(FontCommand),
    Function(Function),
    Text(Text),
    End,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Text {
    Number(String),
    Symbol(String),
    Plain(String),
    Whitespace,
    NewLine,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Operation {
    Plus,
    Minus,
    CDot,
    Ast,
    Star,
    Slash,
    Backslash,
    Times,
    Div,
    LTimes,
    RTimes,
    Bowtie,
    Circ,
    OPlus,
    OTimes,
    ODot,
    Sum,
    Prod,
    Wedge,
    BidWedge,
    Vee,
    BigVee,
    Cap,
    BigCap,
    Cup,
    BigCup,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Misc {
    AsciiFrac,
    LatexFrac,
    Sub,
    Pow,
    Sqrt,
    Root,
    Int,
    OInt,
    Del,
    Grad,
    PlusMinus,
    EmptySet,
    Infty,
    Aleph,
    Therefore,
    Because,
    PLDots,
    PCDots,
    VDots,
    DDots,
    EPipes,
    EQuad,
    Angle,
    Frown,
    Triangle,
    Diamond,
    Square,
    LFloor,
    RFloor,
    LCeiling,
    RCeiling,
    Complex,
    Natural,
    Rational,
    Real,
    Integer,
    LatexText,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Relation {
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
    Prec,
    PrecEq,
    Succ,
    SuccEq,
    In,
    NotIn,
    SubSet,
    SupSet,
    SubSetEq,
    SupSetEq,
    Equiv,
    Cong,
    Approx,
    PropTo,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Logical {
    And,
    Or,
    Not,
    Implies,
    If,
    Iff,
    ForAll,
    Exists,
    Bot,
    Top,
    VDash,
    Models,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Grouping {
    RParen,
    LParen,
    RBracket,
    LBracket,
    RBrace,
    LBrace,
    LAngle,
    RAngle,
    LXPar,
    RXPar,
    Abs,
    Floor,
    Ceil,
    Norm,
    MSep,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Arrow {
    UpArrow,
    DownArrow,
    RightArrow,
    To,
    RightArrowTail,
    TwoHeadRightArrow,
    TwoHeadRightArrowTail,
    MapsTo,
    LeftArrow,
    LeftRightArrow,
    BigRightArrow,
    BigLeftArrow,
    BigLeftRightArrow,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Accent {
    Hat,
    Overline,
    Underline,
    Vec,
    Dot,
    DDot,
    OverSet,
    UnderSet,
    UnderBrace,
    OverBrace,
    Color(String),
    Cancel,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Greek {
    Alpha,
    Beta,
    Gamma,
    BigGamma,
    Delta,
    BigDelta,
    Epsilon,
    VarEpsilon,
    Zeta,
    Eta,
    Theta,
    BigTheta,
    VarTheta,
    Iota,
    Kappa,
    Lambda,
    BigLambda,
    Mu,
    Nu,
    Xi,
    BigXi,
    Pi,
    BigPi,
    Rho,
    Sigma,
    BigSigma,
    Tau,
    Upsilon,
    Phi,
    BigPhi,
    VarPhi,
    Chi,
    Psi,
    BigPsi,
    Omega,
    BigOmega,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Function {
    Sin,
    Cos,
    Tan,
    Sec,
    Csc,
    Cot,
    ArcSin,
    ArcCos,
    ArcTan,
    Sinh,
    Cosh,
    Tanh,
    Sech,
    Csch,
    Coth,
    Exp,
    Log,
    Ln,
    Det,
    Dim,
    Mod,
    Gcd,
    Lcm,
    Lub,
    Glb,
    Min,
    Max,
    F,
    G,
}

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum FontCommand {
    Big,
    BigOutline,
    Cursive,
    TText,
    Fr,
    SansSerif,
}

impl FontCommand {
    pub fn to_string(&self) -> String {
        match self {
            FontCommand::BigOutline => "bbb".to_string(),
            FontCommand::Big => "bb".to_string(),
            FontCommand::SansSerif => "sf".to_string(),
            FontCommand::Fr => "fr".to_string(),
            FontCommand::TText => "tt".to_string(),
            FontCommand::Cursive => "cc".to_string(),
        }
    }
}
