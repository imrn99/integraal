//! integral parameterization code

/// Domain description enum
///
/// This is essentially a discretization of the integrated space.
///
/// Currently, the supported integration domain can only be one-dimensionnal, described using
/// `f64` values (i.e. the type used for further computations). In the future, adding support
/// for higher dimension & generic value type can be considered.
#[derive(Debug, Clone)]
pub enum DomainDescriptor<'a> {
    /// List of values taken by the variable on which we integrate.
    Explicit(&'a [f64]),
    /// Description of a uniform discretization over a certain range of values.
    Uniform {
        /// First value of the range
        start: f64,
        /// Step between each value of the range
        step: f64,
        /// Total number of values
        n_step: usize,
    },
}

/// Function description enum
///
/// This enum is used to provide either the values taken by the function or describe of to compute
/// those.
pub enum FunctionDescriptor {
    /// Direct expression of the function, taking a value of the domain as input & returning the
    /// image of that value through the function.
    Closure(Box<dyn Fn(f64) -> f64>),
    /// List of values taken by the function. The coherence with the domain description must
    /// be ensured by the user in this case.
    Values(Vec<f64>),
}

/// Numerical integration method enum
///
/// # Note on computations
///
/// For the considered integral to be consistent across compute methods for a given description,
/// the left-rectangle (resp. right-rectangle) has to ignore the last (resp. first) value given
/// in the descriptors. This can be visualized in the following example:
///
/// ![COMPARISON](../compute_methods.svg)
///
/// Out of 11 samples, both methods compute the area of 10 polygons. In the case where the domain
/// is uniform & described using a step, the eleventh sample value is useless (for a left-rectangle
/// method).
///
/// The crate assumes that the first and last samples making up your domain corresponds to the
/// limits of the integral. Therefore, these values will be ignored when computing the integral
/// using rectangles.
#[derive(Debug, Clone, Copy)]
pub enum ComputeMethod {
    /// Rectangle method -- [reference](https://en.wikipedia.org/wiki/Riemann_sum)
    RectangleLeft,
    /// Trapezoid method [reference](https://en.wikipedia.org/wiki/Trapezoidal_rule)
    Trapezoid,
    #[cfg(feature = "montecarlo")]
    /// Monte-Carlo method [reference](https://en.wikipedia.org/wiki/Monte_Carlo_integration)
    MonteCarlo {
        /// Number of random number sample per step computation.
        n_sample: usize,
    },
}
