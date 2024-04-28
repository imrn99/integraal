//! module doc

use crate::parameters::ComputeMethod;
use crate::{DomainDescriptor, FunctionDescriptor};

#[derive(Debug)]
pub enum IntegraalError {
    MissingParameters(&'static str),
}

#[derive(Default)]
pub struct Integraal {
    domain: Option<DomainDescriptor>,
    function: Option<FunctionDescriptor>,
    method: Option<ComputeMethod>,
}

impl Integraal {
    pub fn domain(mut self, domain_descriptor: DomainDescriptor) -> Self {
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

    pub fn compute(self) -> Result<f64, IntegraalError> {
        if self.domain.is_none() | self.function.is_none() | self.method.is_none() {
            return Err(IntegraalError::MissingParameters(
                "cannot compute integral - one or more parameter is missing",
            ));
        }
        match &self.method.unwrap() {
            ComputeMethod::Rectangle => {
                todo!()
            }
            ComputeMethod::Trapezoid => {
                todo!()
            }
            ComputeMethod::MonteCarlo { n_sample: _ } => {
                todo!()
            }
        }
    }
}
