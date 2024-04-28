//! crate doc

// --- CUSTOM LINTS

#![warn(clippy::pedantic)]
#![warn(missing_docs)]

// --- MODULES DECLARATION

mod compute_unit;
mod parameters;

// --- RE-EXPORTS

pub use compute_unit::Integraal;
pub use parameters::{ComputeMethod, DomainDescriptor, FunctionDescriptor};
