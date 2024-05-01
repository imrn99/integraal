//! module doc

use crate::parameters::ComputeMethod;
use crate::{DomainDescriptor, FunctionDescriptor};

#[derive(Debug)]
pub enum IntegraalError {
    MissingParameters(&'static str),
    InconsistentParameters(&'static str),
}

#[derive(Default)]
pub struct Integraal<'a> {
    domain: Option<DomainDescriptor<'a>>,
    function: Option<FunctionDescriptor>,
    method: Option<ComputeMethod>,
}

impl<'a> Integraal<'a> {
    pub fn domain(mut self, domain_descriptor: DomainDescriptor<'a>) -> Self {
        self.domain = Some(domain_descriptor);
        self
    }

    pub fn function(mut self, function_descriptor: FunctionDescriptor) -> Self {
        self.function = Some(function_descriptor);
        self
    }

    pub fn method(mut self, compute_method: ComputeMethod) -> Self {
        self.method = Some(compute_method);
        self
    }

    pub fn compute(&mut self) -> Result<f64, IntegraalError> {
        if self.domain.is_none() | self.function.is_none() | self.method.is_none() {
            return Err(IntegraalError::MissingParameters(
                "cannot compute integral - one or more parameter is missing",
            ));
        }
        let res = match (&self.function, &self.domain) {
            (Some(FunctionDescriptor::Values { vals }), Some(DomainDescriptor::Explicit(args))) => {
                if args.len() != vals.len() {
                    return Err(IntegraalError::InconsistentParameters("todo"));
                }
                todo!()
            }
            (
                Some(FunctionDescriptor::Values { vals }),
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
                Some(FunctionDescriptor::Closure { closure }),
                Some(DomainDescriptor::Explicit(args)),
            ) => {
                todo!()
            }
            (
                Some(FunctionDescriptor::Closure { closure }),
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
