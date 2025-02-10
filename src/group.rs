use itertools::Group;

use super::errors::ProofVerifyError;
use super::scalar::{Scalar, ScalarBytes, ScalarBytesFromScalar};
use core::borrow::Borrow;
use core::ops::{Mul, MulAssign};

pub type GroupElement = curve25519_dalek::ristretto::RistrettoPoint;

pub type CompressedGroup = curve25519_dalek::ristretto::CompressedRistretto;

pub trait CompressedGroupExt {
    type Group;
    fn unpack(&self) -> Result<Self::Group, ProofVerifyError>;
}

impl CompressedGroupExt for CompressedGroup {
    type Group = curve25519_dalek::ristretto::RistrettoPoint;

    fn unpack(&self) -> Result<Self::Group, ProofVerifyError> {
        self.decompress()
            .ok_or_else(|| ProofVerifyError::DecompressionError(self.to_bytes()))
    }
}

pub const GROUP_BASEPOINT_COMPRESSED: CompressedGroup =
    curve25519_dalek::constants::RISTRETTO_BASEPOINT_COMPRESSED;

impl<'b> MulAssign<&'b Scalar> for GroupElement  {
    fn mul_assign(&mut self, scalar: &'b Scalar) {
        let result = (self as &GroupElement) * Scalar::decompress_scalar(scalar);
        *self = result;
    }
}

impl<'b> Mul<&'b Scalar> for &GroupElement {
    type Output = GroupElement;

    fn mul(self, scalar: &'b Scalar) -> GroupElement {
        self * Scalar::decompress_scalar(scalar)
    }
}

impl<'b> Mul<&'b GroupElement> for &Scalar {
    type Output = GroupElement;

    fn mul(self, point: &'b GroupElement) -> GroupElement {
        Scalar::decompress_scalar(self) * point
    }
}

pub trait VartimeMultiscalarMul {
    type Scalar;
    fn vartime_multiscalar_mul<I,J>(scalars: I, points: J) -> Self
    where 
    I:IntoIterator,
    I::Item
    
}