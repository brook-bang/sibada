pub type GroupElement = curve25519_dalek::ristretto::RistrettoPoint;
pub type CompressedGroup = curve25519_dalek::ristretto::CompressedRistretto;
pub const GROUP_BASEPOINT_COMPRESSED: CompressedGroup = curve25519_dalek::constants::RISTRETTO_BASEPOINT_COMPRESSED;