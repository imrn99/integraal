//! module doc

pub trait DomainScalar:
    Clone + Copy + std::ops::Sub<Output = Self> + num::Float + num::Signed + num::FromPrimitive
{
}

impl<
        X: Clone + Copy + std::ops::Sub<Output = Self> + num::Float + num::Signed + num::FromPrimitive,
    > DomainScalar for X
{
}

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

pub trait IntegratedScalar: Clone + Copy + std::iter::Sum {}
