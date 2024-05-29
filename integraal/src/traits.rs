//! Common traits implementation

/// Scalar value trait.
///
/// This trait is automatically implemented for all types implementing its requirements.
pub trait Scalar:
    Clone
    + Copy
    + num::Float
    + num::Signed
    + num::FromPrimitive
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::iter::Sum
{
}

impl<
        X: Clone
            + Copy
            + num::Float
            + num::Signed
            + num::FromPrimitive
            + std::ops::Sub<Output = Self>
            + std::ops::Mul<Output = Self>
            + std::iter::Sum,
    > Scalar for X
{
}
