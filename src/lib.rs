//! crate doc

// --- CUSTOM LINTS

// if compiling using nightly, enable auto feature-gates documentation
#![cfg_attr(nightly, doc_auto_cfg)]
// more lints
#![warn(clippy::pedantic)]
#![warn(missing_docs)]

// --- MODULES DECLARATION

mod compute_unit;
mod parameters;

// --- RE-EXPORTS

pub use compute_unit::Integraal;
pub use parameters::{ComputeMethod, DomainDescriptor, FunctionDescriptor};
