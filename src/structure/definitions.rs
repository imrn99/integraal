//! module doc
//!

// ------ IMPORTS

use crate::{ComputeMethod, DomainDescriptor, FunctionDescriptor};

// ------ CONTENT

/// Integral error
#[derive(Debug)]
pub enum IntegraalError {
    /// One or more parameters are missing.
    MissingParameters(&'static str),
    /// Specified parameters are conflicting or ambiguous.
    InconsistentParameters(&'static str),
}

/// Main integral computation structure
///
/// This structure is used as the entrypoint for integral definition and computation. It follows
/// a pseudo-builder patterns where the function description is reset after a computation.
///
/// # Usage
///
/// ## Components
///
/// The structure is made up of three components that are used to describe the integral the user
/// wishes to compute:
/// - a [`DomainDescriptor`] instance, used to describe the space over which the integral span
/// - a [`FunctionDescriptor`] instance, used to describe the integrated function
/// - a [`ComputeMethod`] instance, used to choose which numerical integration method will be used
///   for computation
///
/// In the future, another object might be included to control the execution backend.
///
/// ## Example
///
/// ```rust
/// # use integraal::{DomainDescriptor, ComputeMethod, FunctionDescriptor, Integraal, IntegraalError};
/// # fn main() {
/// // describe domain, function & computation method
/// let domain = DomainDescriptor::Uniform {
///     start: 0.0,
///     step: 0.00001,
///     n_step: 100_001,
/// };
///
/// // decribe the function and numerical integration method
/// let function = FunctionDescriptor::Closure(Box::new(|x: f64| 2.0 * x));
/// let method = ComputeMethod::Trapezoid;
///
/// // build the integral & compute it
/// let mut integral = Integraal::default();
/// integral.domain(domain).function(function).method(method);
/// assert!(integral.compute().is_ok())
/// # }
/// ```
#[derive(Default)]
pub struct Integraal<'a> {
    pub(crate) domain: Option<DomainDescriptor<'a>>,
    pub(crate) function: Option<FunctionDescriptor>,
    pub(crate) method: Option<ComputeMethod>,
}
