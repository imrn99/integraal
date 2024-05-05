//! main structure and computation code

use crate::parameters::ComputeMethod;
use crate::{DomainDescriptor, FunctionDescriptor};

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
/// todo!()
/// ```
#[derive(Default)]
pub struct Integraal<'a> {
    domain: Option<DomainDescriptor<'a>>,
    function: Option<FunctionDescriptor>,
    method: Option<ComputeMethod>,
}

impl<'a> Integraal<'a> {
    /// Setter
    pub fn domain(&mut self, domain_descriptor: DomainDescriptor<'a>) -> &mut Self {
        self.domain = Some(domain_descriptor);
        self
    }

    /// Setter
    pub fn function(&mut self, function_descriptor: FunctionDescriptor) -> &mut Self {
        self.function = Some(function_descriptor);
        self
    }

    /// Setter
    pub fn method(&mut self, compute_method: ComputeMethod) -> &mut Self {
        self.method = Some(compute_method);
        self
    }

    #[allow(clippy::missing_errors_doc)]
    /// Main computation method
    ///
    /// This method attempts to compute the integral. If it is successful, it will clear the
    /// internal [`FunctionDescriptor`] object before returning the result.
    ///
    /// # Return / Errors
    ///
    /// This method returns a `Result` taking the following values:
    /// - `Ok(f64)` -- The computation was successfuly done
    /// - `Err(IntegraalError)` -- The computation failed for the reason specified by the enum.
    pub fn compute(&mut self) -> Result<f64, IntegraalError> {
        if self.domain.is_none() | self.function.is_none() | self.method.is_none() {
            return Err(IntegraalError::MissingParameters(
                "cannot compute integral - one or more parameter is missing",
            ));
        }
        let res = match (&self.function, &self.domain) {
            (Some(FunctionDescriptor::Values(vals)), Some(DomainDescriptor::Explicit(args))) => {
                if args.len() != vals.len() {
                    return Err(IntegraalError::InconsistentParameters("todo"));
                }
                todo!()
            }
            (
                Some(FunctionDescriptor::Values(vals)),
                Some(DomainDescriptor::Uniform {
                    start: _,
                    step,
                    n_step,
                }),
            ) => {
                if *n_step != vals.len() {
                    return Err(IntegraalError::InconsistentParameters("todo"));
                }
                todo!()
            }
            (
                Some(FunctionDescriptor::Closure(closure)),
                Some(DomainDescriptor::Explicit(args)),
            ) => {
                todo!()
            }
            (
                Some(FunctionDescriptor::Closure(closure)),
                Some(DomainDescriptor::Uniform {
                    start,
                    step,
                    n_step,
                }),
            ) => {
                // compute args
                match &self.method {
                    Some(ComputeMethod::Rectangle) => (0..*n_step)
                        .map(|step_id| {
                            let x = start + step * step_id as f64;
                            closure(x) * step
                        })
                        .sum(),
                    Some(ComputeMethod::Trapezoid) => (1..*n_step)
                        .map(|step_id| {
                            let x1 = start + step * (step_id - 1) as f64;
                            let x2 = start + step * step_id as f64;
                            let y1 = closure(x1);
                            let y2 = closure(x2);
                            step * (y1.min(y2) + (y1 - y2).abs() / 2.0)
                        })
                        .sum(),
                    #[cfg(feature = "montecarlo")]
                    Some(ComputeMethod::MonteCarlo { n_sample: _ }) => {
                        todo!()
                    }
                    None => unreachable!(),
                }
            }
            (_, _) => unreachable!(),
        };
        self.function = None;
        Ok(res)
    }
}
