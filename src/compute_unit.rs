//! module doc

use crate::parameters::ComputeMethod;
use crate::{DomainDescriptor, FunctionDescriptor};

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

    pub fn compute(&self) -> f64 {
        todo!()
    }
}
