//! main method implementations

// ------ IMPORTS

use crate::{
    ComputeMethod, DomainDescriptor, FunctionDescriptor, Integraal, IntegraalError, Scalar,
};

// ------ CONTENT

impl<'a, X: Scalar> Integraal<'a, X> {
    /// Set the domain descriptor.
    #[must_use = "unused builder struct - please remove this call"]
    pub fn domain(mut self, domain_descriptor: DomainDescriptor<'a, X>) -> Self {
        self.domain = Some(domain_descriptor);
        self
    }

    /// Set the function descriptor.
    #[must_use = "unused builder struct - please remove this call"]
    pub fn function(mut self, function_descriptor: FunctionDescriptor<X>) -> Self {
        self.function = Some(function_descriptor);
        self
    }

    /// Set the numerical integration method.
    #[must_use = "unused builder struct - please remove this call"]
    pub fn method(mut self, compute_method: ComputeMethod) -> Self {
        self.method = Some(compute_method);
        self
    }

    #[allow(clippy::missing_errors_doc, clippy::missing_panics_doc)]
    /// This method attempts to compute the integral. If it is successful, it will clear the
    /// internal [`FunctionDescriptor`] object before returning the result.
    ///
    /// # Return / Errors
    ///
    /// This method returns a `Result` taking the following values:
    /// - `Ok(X: Scalar)` -- The computation was successfuly done
    /// - `Err(IntegraalError)` -- The computation failed for the reason specified by the enum.
    pub fn compute(&mut self) -> Result<X, IntegraalError> {
        // ensure all data is defined
        if self.domain.is_none() | self.function.is_none() | self.method.is_none() {
            return Err(IntegraalError::MissingParameters(
                "one or more parameter is missing",
            ));
        }

        let Some(method) = &self.method else {
            unreachable!()
        };

        let Some(domain) = &self.domain else {
            unreachable!()
        };

        #[rustfmt::skip]
        let res = match (&self.function, &self.domain) {
            // function descriptor -- values
            // domain descriptor   -- explicit
            (
                Some(FunctionDescriptor::Values(vals)),
                Some(DomainDescriptor::Explicit(args))
            ) => values_explicit_arm(vals, args, method)?,
            // function descriptor -- values
            // domain descriptor   -- uniform
            (
                Some(FunctionDescriptor::Values(vals)),
                Some(DomainDescriptor::Uniform { .. })
            ) => values_uniform_arm(vals, domain, method)?,
            // function descriptor -- closure
            // domain descriptor   -- explicit
            (
                Some(FunctionDescriptor::Closure(closure)),
                Some(DomainDescriptor::Explicit(args)),
            ) => closure_explicit_arm(closure, args, method)?,
            // function descriptor -- closure
            // domain descriptor   -- uniform
            (
                Some(FunctionDescriptor::Closure(closure)),
                Some(DomainDescriptor::Uniform { .. }),
            ) => closure_uniform_arm(closure, domain, method)?,
            (_, _) => unreachable!(),
        };

        self.function = None; // is this really useful? we could wire returns directly using `?` if this wasn't here
        Ok(res)
    }
}

// ---

fn values_explicit_arm<X: Scalar>(
    vals: &[X],
    args: &[X],
    method: &ComputeMethod,
) -> Result<X, IntegraalError> {
    if args.len() != vals.len() {
        return Err(IntegraalError::InconsistentParameters(
            "function and domain value slices have different lengthes",
        ));
    }
    let n_sample = args.len();

    // because the domain may be not uniform, we have to compute step values
    let res = match method {
        ComputeMethod::RectangleLeft => (1..n_sample)
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                vals[idx - 1] * step
            })
            .sum(),
        ComputeMethod::RectangleRight => (1..n_sample)
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                vals[idx] * step
            })
            .sum(),
        ComputeMethod::Trapezoid => (1..n_sample)
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                let y1 = vals[idx - 1];
                let y2 = vals[idx];
                (y1.min(y2) + num_traits::abs(y1 - y2) / X::from_f32(2.0).unwrap()) * step
            })
            .sum(),
        ComputeMethod::SimpsonsThird => {
            todo!();
        }
        #[cfg(feature = "montecarlo")]
        ComputeMethod::MonteCarlo { n_sample: _ } => {
            todo!()
        }
    };

    Ok(res)
}

fn values_uniform_arm<X: Scalar>(
    vals: &[X],
    domain: &DomainDescriptor<X>,
    method: &ComputeMethod,
) -> Result<X, IntegraalError> {
    let DomainDescriptor::Uniform {
        start: _,
        step,
        n_step,
    } = domain
    else {
        unreachable!()
    };

    if *n_step != vals.len() {
        return Err(IntegraalError::InconsistentParameters(
            "provided function and domain value slices have different lengthes",
        ));
    }

    // we can use the uniform domain's step & number of step to compute areas
    let res = match method {
        ComputeMethod::RectangleLeft => {
            // ignore the last value since its a left rule
            (0..*n_step - 1).map(|step_id| vals[step_id] * *step).sum()
        }
        ComputeMethod::RectangleRight => {
            // ignore the last value since its a left rule
            (1..*n_step).map(|step_id| vals[step_id] * *step).sum()
        }
        ComputeMethod::Trapezoid => (1..*n_step)
            .map(|step_id| {
                let y1 = vals[step_id - 1];
                let y2 = vals[step_id];
                (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * *step
            })
            .sum(),
        ComputeMethod::SimpsonsThird => {
            todo!();
        }
        #[cfg(feature = "montecarlo")]
        ComputeMethod::MonteCarlo { n_sample: _ } => {
            todo!()
        }
    };

    Ok(res)
}

#[allow(clippy::unnecessary_wraps)]
fn closure_explicit_arm<X: Scalar>(
    closure: impl Fn(X) -> X,
    args: &[X],
    method: &ComputeMethod,
) -> Result<X, IntegraalError> {
    let res = match method {
        ComputeMethod::RectangleLeft => (1..args.len())
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                closure(args[idx - 1]) * step
            })
            .sum(),
        ComputeMethod::RectangleRight => (1..args.len())
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                closure(args[idx]) * step
            })
            .sum(),
        ComputeMethod::Trapezoid => (1..args.len())
            .map(|idx| {
                let step = args[idx] - args[idx - 1];
                let y1 = closure(args[idx - 1]);
                let y2 = closure(args[idx]);
                (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * step
            })
            .sum(),
        ComputeMethod::SimpsonsThird => {
            todo!();
        }
        #[cfg(feature = "montecarlo")]
        ComputeMethod::MonteCarlo { n_sample: _ } => {
            todo!()
        }
    };
    Ok(res)
}

#[allow(clippy::unnecessary_wraps)]
fn closure_uniform_arm<X: Scalar>(
    closure: impl Fn(X) -> X,
    domain: &DomainDescriptor<X>,
    method: &ComputeMethod,
) -> Result<X, IntegraalError> {
    let DomainDescriptor::Uniform {
        start,
        step,
        n_step,
    } = domain
    else {
        unreachable!()
    };

    // compute args
    let res = match method {
        ComputeMethod::RectangleLeft => (0..*n_step - 1)
            .map(|step_id| {
                let x = *start + *step * X::from_usize(step_id).unwrap();
                closure(x) * *step
            })
            .sum(),
        ComputeMethod::RectangleRight => (1..*n_step)
            .map(|step_id| {
                let x = *start + *step * X::from_usize(step_id).unwrap();
                closure(x) * *step
            })
            .sum(),
        ComputeMethod::Trapezoid => (1..*n_step)
            .map(|step_id| {
                let x1 = *start + *step * X::from_usize(step_id - 1).unwrap();
                let x2 = *start + *step * X::from_usize(step_id).unwrap();
                let y1 = closure(x1);
                let y2 = closure(x2);
                (y1.min(y2) + (y1 - y2).abs() / X::from_f32(2.0).unwrap()) * *step
            })
            .sum(),
        ComputeMethod::SimpsonsThird => {
            todo!();
        }
        #[cfg(feature = "montecarlo")]
        ComputeMethod::MonteCarlo { n_sample: _ } => {
            todo!()
        }
    };

    Ok(res)
}
