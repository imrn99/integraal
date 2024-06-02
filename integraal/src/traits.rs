//! Common traits implementation

use std::fmt::Debug;

/// Scalar value trait.
///
/// This trait is automatically implemented for all types implementing its requirements.
pub trait Scalar:
    Clone
    + Copy
    + Debug
    + num_traits::Float
    + num_traits::Signed
    + num_traits::FromPrimitive
    + std::ops::Sub<Output = Self>
    + std::ops::Mul<Output = Self>
    + std::iter::Sum
{
}

impl<
        X: Clone
            + Copy
            + Debug
            + num_traits::Float
            + num_traits::Signed
            + num_traits::FromPrimitive
            + std::ops::Sub<Output = Self>
            + std::ops::Mul<Output = Self>
            + std::iter::Sum,
    > Scalar for X
{
}
