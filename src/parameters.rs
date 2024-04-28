//! module doc

pub struct DomainDescriptor<T> {
    pub(super) limits: Option<(T, T)>,
    pub(super) step: Option<T>,
    pub(super) n_step: Option<usize>,
}

pub enum FunctionDescriptor<T> {
    Closure { closure: Box<dyn Fn(T) -> T> },
    Values { vals: Vec<T> },
}
