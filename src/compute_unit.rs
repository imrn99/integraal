//! module doc

use crate::parameters::ComputeMethod;
use crate::{DomainDescriptor, FunctionDescriptor};

pub struct Integraal {
    domain: Option<DomainDescriptor>,
    function: Option<FunctionDescriptor>,
    method: Option<ComputeMethod>,
}
