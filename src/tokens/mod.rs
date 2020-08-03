pub mod constants;
pub mod mappings;

#[derive(Debug, Clone)]
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
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Misc {
    AsciiFrac,
    LatexFrac,
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
    AsciiText,
    LatexText,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
pub enum Grouping {
    RParen,
    LParen,
    RBrace,
    LBrace,
    RCurl,
    LCurl,
    LAngle,
    RAngle,
    LXPar,
    RXPar,
    Abs,
    Floor,
    Ceil,
    Norm,
}

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
    Color,
    Cancel,
}

#[derive(Debug, Clone)]
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
    Omega,
}

#[derive(Debug, Clone)]
pub enum FontCommand {
    Big,
    BigOutline,
    Cursive,
    TText,
    Fr,
    SansSerif,
}
