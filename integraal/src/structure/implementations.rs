//! main method implementations

// ------ IMPORTS

use crate::{
    ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal, IntegraalError, Scalar,
};
use num::abs;
use std::ops::Deref;

// ------ CONTENT

impl<'a, X: Scalar> Integraal<'a, X> {
    /// Set the domain descriptor.
    pub fn domain(&mut self, domain_descriptor: DomainDescriptor<'a, X>) -> &mut Self {
        self.domain = Some(domain_descriptor);
        self
    }

    /// Set the function descriptor.
    pub fn function(&mut self, function_descriptor: FunctionDescriptor<X>) -> &mut Self {
        self.function = Some(function_descriptor);
        self
    }

    /// Set the numerical integration method.
    pub fn method(&mut self, compute_method: ComputeMethod) -> &mut Self {
        self.method = Some(compute_method);
        self
    }

    #[allow(
        clippy::missing_errors_doc,
        clippy::missing_panics_doc,
        clippy::too_many_lines
    )]
    /// This method attempts to compute the integral. If it is successful, it will clear the
    /// internal [`FunctionDescriptor`] object before returning the result.
    ///
    /// # Return / Errors
    ///
    /// This method returns a `Result` taking the following values:
    /// - `Ok(X: Scalar)` -- The computation was successfuly done
    /// - `Err(IntegraalError)` -- The computation failed for the reason specified by the enum.
    pub fn compute(&mut self) -> Result<X, IntegraalError> {
        if self.domain.is_none() | self.function.is_none() | self.method.is_none() {
            return Err(IntegraalError::MissingParameters(
                "one or more parameter is missing",
            ));
        }
        let res = match (&self.function, &self.domain) {
            (Some(FunctionDescriptor::Values(vals)), Some(DomainDescriptor::Explicit(args))) => {
                if args.len() != vals.len() {
                    return Err(IntegraalError::InconsistentParameters(
                        "provided function and domain value slices have different lengthes",
                    ));
                }
                let n_sample = args.len();

                // because the domain may be not uniform, we have to compute step values
                match &self.method {
                    Some(ComputeMethod::RectangleLeft) => (1..n_sample)
                        .map(|idx| {
                            let step = args[idx] - args[idx - 1];
                            vals[idx - 1] * step
                        })
                        .sum(),
                    Some(ComputeMethod::RectangleRight) => (1..n_sample)
                        .map(|idx| {
                            let step = args[idx] - args[idx - 1];
                            vals[idx] * step
                        })
                        .sum(),
                    Some(ComputeMethod::Trapezoid) => (1..n_sample)
                        .map(|idx| {
                            let step = args[idx] - args[idx - 1];
                            let y1 = vals[idx - 1];
                            let y2 = vals[idx];
                            (y1.min(y2) + abs(y1 - y2) / X::from_f32(2.0).unwrap()) * step
                        })
                        .sum(),
                    #[cfg(feature = "montecarlo")]
                    Some(ComputeMethod::MonteCarlo { n_sample: _ }) => {
                        todo!()
                    }
                    None => unreachable!(),
                }
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
                    return Err(IntegraalError::InconsistentParameters(
                        "provided function and domain value slices have different lengthes",
                    ));
                }

                // we can use the uniform domain's step & number of step to compute areas
                match &self.method {
                    Some(ComputeMethod::RectangleLeft) => {
                        // ignore the last value since its a left rule
                        (0..*n_step - 1).map(|step_id| vals[step_id] * *step).sum()
                    }
                    Some(ComputeMethod::RectangleRight) => {
                        // ignore the last value since its a left rule
                        (1..*n_step).map(|step_id| vals[step_id] * *step).sum()
                    }
                    Some(ComputeMethod::Trapezoid) => (1..*n_step)
                        .map(|step_id| {
                            let y1 = vals[step_id - 1];
                            let y2 = vals[step_id];
                            (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * *step
                        })
                        .sum(),
                    #[cfg(feature = "montecarlo")]
                    Some(ComputeMethod::MonteCarlo { n_sample: _ }) => {
                        todo!()
                    }
                    None => unreachable!(),
                }
            }
            (
                Some(FunctionDescriptor::Closure(closure)),
                Some(DomainDescriptor::Explicit(args)),
            ) => match &self.method {
                Some(ComputeMethod::RectangleLeft) => (1..args.len())
                    .map(|idx| {
                        let step = args[idx] - args[idx - 1];
                        closure(args[idx - 1]) * step
                    })
                    .sum(),
                Some(ComputeMethod::RectangleRight) => (1..args.len())
                    .map(|idx| {
                        let step = args[idx] - args[idx - 1];
                        closure(args[idx]) * step
                    })
                    .sum(),
                Some(ComputeMethod::Trapezoid) => (1..args.len())
                    .map(|idx| {
                        let step = args[idx] - args[idx - 1];
                        let y1 = closure.deref()(args[idx - 1]);
                        let y2 = closure(args[idx]);
                        (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * step
                    })
                    .sum(),
                #[cfg(feature = "montecarlo")]
                Some(ComputeMethod::MonteCarlo { n_sample: _ }) => {
                    todo!()
                }
                None => unreachable!(),
            },
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
                    Some(ComputeMethod::RectangleLeft) => (0..*n_step - 1)
                        .map(|step_id| {
                            let x = *start + *step * X::from_usize(step_id).unwrap();
                            closure(x) * *step
                        })
                        .sum(),
                    Some(ComputeMethod::RectangleRight) => (1..*n_step)
                        .map(|step_id| {
                            let x = *start + *step * X::from_usize(step_id).unwrap();
                            closure(x) * *step
                        })
                        .sum(),
                    Some(ComputeMethod::Trapezoid) => (1..*n_step)
                        .map(|step_id| {
                            let x1 = *start + *step * X::from_usize(step_id - 1).unwrap();
                            let x2 = *start + *step * X::from_usize(step_id).unwrap();
                            let y1 = closure.deref()(x1);
                            let y2 = closure(x2);
                            (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * *step
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

    #[allow(
        clippy::missing_errors_doc,
        clippy::missing_panics_doc,
        clippy::too_many_lines
    )]
    /// This method attempts to compute the numerical error one can expect from the approximation.
    ///
    /// TODO: add ref to error formulae
    ///
    /// # Return / Errors
    ///
    /// This method returns a `Result` taking the following values:
    /// - `Ok(X: Scalar)` -- The computation was successfuly done
    /// - `Err(IntegraalError)` -- The computation failed for the reason specified by the enum.
    pub fn compute_error(&mut self) -> Result<X, IntegraalError> {
        if self.domain.is_none() | self.function.is_none() | self.method.is_none() {
            return Err(IntegraalError::MissingParameters(
                "one or more parameter is missing",
            ));
        }
        todo!()
    }
}
