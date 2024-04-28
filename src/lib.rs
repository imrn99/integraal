//! crate doc

// --- CUSTOM LINTS

#![warn(clippy::pedantic)]
#![warn(missing_docs)]

// --- MODULES DECLARATION

mod parameters;

// --- RE-EXPORTS

pub use parameters::{DomainDescriptor, FunctionDescriptor};
