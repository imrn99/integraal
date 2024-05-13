//! module doc

/// Scalar domain value trait.
///
/// This trait is automatically implemented for all types implementing its requirements.
pub trait DomainScalar:
    Clone + Copy + std::ops::Sub<Output = Self> + num::Float + num::Signed + num::FromPrimitive
{
}

impl<
        X: Clone + Copy + std::ops::Sub<Output = Self> + num::Float + num::Signed + num::FromPrimitive,
    > DomainScalar for X
{
}

/// Scalar image value trait.
///
/// This trait is automatically implemented for all types implementing its requirements.
pub trait ImageScalar<X: DomainScalar, W: IntegratedScalar>:
    Clone
    + Copy
    + std::ops::Mul<X, Output = W>
    + std::ops::Sub<Output = Self>
    + num::Float
    + num::Signed
    + num::FromPrimitive
{
}

impl<
        X: DomainScalar,
        Y: Clone
            + Copy
            + std::ops::Mul<X, Output = W>
            + std::ops::Sub<Output = Self>
            + num::Float
            + num::Signed
            + num::FromPrimitive,
        W: IntegratedScalar,
    > ImageScalar<X, W> for Y
{
}

/// Scalar post-integration value trait.
///
/// This trait is automatically implemented for all types implementing its requirements.
pub trait IntegratedScalar: Clone + Copy + std::iter::Sum {}

impl<W: Clone + Copy + std::iter::Sum> IntegratedScalar for W {}
