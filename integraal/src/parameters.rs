//! integral parameterization code

use crate::Scalar;

/// Domain description enum
///
/// This represents a discretization of the integrated space.
///
/// Only 1D domains are currently supported. the type used for values must implement [`Scalar`]; the trait
/// is automatially implemented for types satisfying its requirements. In the future, adding support for
/// higher dimension can be considered.
#[derive(Debug, Clone)]
pub enum DomainDescriptor<X: Scalar> {
    /// List of values taken by the variable on which we integrate.
    Explicit(Vec<X>),
    /// Description of a uniform discretization over a certain range of values.
    Uniform {
        /// First value of the range
        start: X,
        /// Step between each value of the range
        step: X,
        /// Total number of values
        n_step: usize,
    },
}

/// Function description enum
///
/// This holds information about the function's values, as either explicit values or a closure that
/// can be used to compute those.
pub enum FunctionDescriptor<X>
where
    X: Scalar,
{
    /// Direct expression of the function, taking a value of the domain as input & returning the
    /// image of that value.
    Closure(Box<dyn Fn(X) -> X>),
    /// List of values taken by the function. An error will be raised at computation if the length
    /// of the list isn't consistent with the domain descriptor.
    Values(Vec<X>),
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
    /// Rectangle method, using the left rule --
    /// [reference](https://en.wikipedia.org/wiki/Riemann_sum#Left_rule)
    RectangleLeft,
    /// Rectangle method, using the right rule --
    /// [reference](https://en.wikipedia.org/wiki/Riemann_sum#Right_rule)
    RectangleRight,
    /// Trapezoid method -- [reference](https://en.wikipedia.org/wiki/Trapezoidal_rule)
    Trapezoid,
    /// Simpson's rule(s), the exact rule applied depends on integral definition --
    /// [reference](https://en.wikipedia.org/wiki/Simpson%27s_rule)
    Simpson,
    /// Boole's method -- [reference](https://en.wikipedia.org/wiki/Boole%27s_rule#Composite_Boole's_Rule)
    #[cfg(feature = "boole")]
    Boole {
        /// Force the computation by truncating inputs to fit method requirements
        force: bool,
    },
    /// Romberg's method -- [reference](https://en.wikipedia.org/wiki/Romberg%27s_method#Implementation)
    #[cfg(feature = "romberg")]
    Romberg {
        /// Maximum number of iteration done by the algorithm
        max_steps: usize,
    },
    #[cfg(feature = "montecarlo")]
    /// Monte-Carlo method -- [reference](https://en.wikipedia.org/wiki/Monte_Carlo_integration)
    MonteCarlo {
        /// Number of samples per step computation.
        n_sample: usize,
    },
}
