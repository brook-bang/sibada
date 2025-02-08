use std::ops::MulAssign;

use crate::{errors::ProofVerifyError, scalar::Scalar};

pub type GroupElement = curve25519_dalek::ristretto::RistrettoPoint;
pub type CompressedGroup = curve25519_dalek::ristretto::CompressedRistretto;

pub trait CompressedGroupExt {
    type Group;
    fn unpack(&self) -> Result<Self::Group,ProofVerifyError>;
}

impl CompressedGroupExt for CompressedGroup {
    type Group = curve25519_dalek::ristretto::RistrettoPoint;
    
    fn unpack(&self) -> Result<Self::Group,ProofVerifyError> {
        self.decompress().ok_or_else(|| ProofVerifyError::DecompressionError(self.to_bytes()))
    }    
}

pub const GROUP_BASEPOINT_COMPRESSED: CompressedGroup = curve25519_dalek::constants::RISTRETTO_BASEPOINT_COMPRESSED;

impl<'b> MulAssign<&'b Scalar> for GroupElement  {
    fn mul_assign(&mut self, rhs: &'b Scalar) {
        todo!()
    }
}