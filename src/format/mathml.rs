use crate::tokens::Greek;

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
