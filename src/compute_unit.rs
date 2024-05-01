//! module doc

use crate::parameters::ComputeMethod;
use crate::{DomainDescriptor, FunctionDescriptor};

#[derive(Debug)]
pub enum IntegraalError {
    MissingParameters(&'static str),
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
        let res = match self.method {
            Some(ComputeMethod::Rectangle) => {
                todo!()
            }
            Some(ComputeMethod::Trapezoid) => {
                todo!()
            }
            Some(ComputeMethod::MonteCarlo { n_sample: _ }) => {
                todo!()
            }
            None => unreachable!(),
        };
        self.function = None;
        Ok(res)
    }
}
