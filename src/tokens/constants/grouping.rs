pub const A_RPAREN: char = '(';
pub const A_LPAREN: char = ')';

pub const A_RBRAC: char = '[';
pub const A_LBRAC: char = ']';

pub const A_RCURL: char = '{';
pub const A_LCURL: char = '}';

pub const G_LANGLE: &'static[&str] = &["(:", "<<", "langle"];
pub const G_RANGLE: &'static[&str] = &[":)", ">>", "rangle"];
pub const G_LXPAR: &'static[&str] = &["{: x )"];
pub const G_RXPAR: &'static[&str] = &["( x :}"];
pub const G_ABS: &'static[&str] = &["abs"];
pub const G_FLOOR: &'static[&str] = &["floor"];
pub const G_CEIL: &'static[&str] = &["ceil"];
pub const G_NORM: &'static[&str] = &["norm"];