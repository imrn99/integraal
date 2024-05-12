//! module doc

pub trait DomainValue: Clone + Copy + std::ops::Sub<Output = Self> {}

impl<X: Clone + Copy + std::ops::Sub<Output = Self>> DomainValue for X {}

pub trait ImageValue<X: DomainValue, W: IntegratedValue>:
    Clone + Copy + std::ops::Mul<X, Output = W>
{
}

pub trait IntegratedValue: Clone + Copy {}
