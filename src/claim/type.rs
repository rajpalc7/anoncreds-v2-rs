use serde_repr::{Deserialize_repr, Serialize_repr};

/// The claim type
#[derive(Copy, Clone, Debug, Eq, PartialEq, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
pub enum ClaimType {
    /// The case where its none of the others
    Unknown = 0,
    /// Hashed claims
    Hashed = 1,
    /// Numeric claims
    Number = 2,
    /// Scalar based claims
    Scalar = 3,
    /// Revocation based claims
    Revocation = 4,
}