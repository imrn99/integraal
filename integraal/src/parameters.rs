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
#[derive(Debug, Clone, Copy)]
pub enum ComputeMethod {
    /// Rectangle method -- [reference](https://en.wikipedia.org/wiki/Riemann_sum)
    Rectangle,
    /// Trapezoid method [reference](https://en.wikipedia.org/wiki/Trapezoidal_rule)
    Trapezoid,
    #[cfg(feature = "montecarlo")]
    /// MonteCarlo method [reference](https://en.wikipedia.org/wiki/Monte_Carlo_integration)
    MonteCarlo { n_sample: usize },
}
