use core::{
    fmt::Display,
    fmt::{self, Debug},
};
use std::default;

#[derive(Debug, Default)]
pub enum ProofVerifyError {
    #[default]
    InternalError,
    DecompressionError([u8; 32]),
}

impl Display for ProofVerifyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ProofVerifyError::DecompressionError(bytes) => write!(
                f,
                "Compressed group element failed to decompress: {bytes:?}",
            ),
            ProofVerifyError::InternalError => {
                write!(f, "Proof verification failed",)
            }
        }
    }
}

#[derive(Clone,Debug,Eq,PartialEq)]
pub enum R1CSError {
    NonPowerofTwoCons,
    NonPowerofTwoVars,
    InvalidNumberOfInputs,
    InvalidNumberOfVars,
    InvalidScalar,
    InvalidIndex,
}