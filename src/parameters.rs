//! module doc

pub enum DomainDescriptor<'a> {
    Explicit(&'a [f64]),
    Uniform {
        start: f64,
        step: f64,
        n_step: usize,
    },
}

pub enum FunctionDescriptor {
    Closure { closure: Box<dyn Fn(f64) -> f64> },
    Values { vals: Vec<f64> },
}

pub enum ComputeMethod {
    Rectangle,
    Trapezoid,
    #[cfg(feature = "montecarlo")]
    MonteCarlo {
        n_sample: usize,
    },
}
