//! module doc

pub struct DomainDescriptor {
    pub(super) limits: Option<(f64, f64)>,
    pub(super) step: Option<f64>,
    pub(super) n_step: Option<usize>,
}

pub enum FunctionDescriptor {
    Closure { closure: Box<dyn Fn(f64) -> f64> },
    Values { vals: Vec<f64> },
}

pub enum ComputeMethod {
    Rectangle,
    Trapezoid,
    MonteCarlo { n_sample: usize },
}
