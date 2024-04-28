//! module doc

#[derive(Default)]
pub struct DomainDescriptor {
    pub(super) boundaries: Option<(f64, f64)>,
    pub(super) step: Option<f64>,
    pub(super) n_step: Option<usize>,
}

impl DomainDescriptor {
    pub fn boundaries(mut self, boundaries: (f64, f64)) -> Self {
        self.boundaries = Some(boundaries);
        self
    }

    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    pub fn n_step(mut self, n_step: usize) -> Self {
        self.n_step = Some(n_step);
        self
    }
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
